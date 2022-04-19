#![feature(io_error_more)]

mod cli;
mod macros;

use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::ArgMatches;
use cli::args;
use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    DesktopLanguageRequester,
};
use itertools::Itertools;
use log::{debug, error, info, trace, LevelFilter};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

use brick::{
    error::{Error, Result},
    packer::{tar::TarPacker, Packer},
    ArchiveFormat, CompressionLevel,
};

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub(crate) static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader = fluent_language_loader!();
    let requested_languages = DesktopLanguageRequester::requested_languages();
    let _result = i18n_embed::select(&loader, &Localizations, &requested_languages);
    loader
});

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    let app = cli::app();

    let matches = app.get_matches();

    let log_level = match 3 + matches.occurrences_of("verbose") - matches.occurrences_of("quiet") {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        5 => LevelFilter::Trace,
        _ => unreachable!(),
    };

    env_logger::Builder::new()
        .filter(None, log_level)
        .parse_default_env()
        .init();

    match matches.subcommand() {
        Some(("info", _sub_matches)) => todo!(),
        Some(("pack", sub_matches)) => pack(sub_matches)?,
        Some(("unpack", _sub_matches)) => todo!(),
        Some(_) => todo!(),
        None => todo!(),
    }

    Ok(())
}

fn pack(sub_matches: &ArgMatches) -> Result<()> {
    if sub_matches.occurrences_of(args::COMPRESSION_LEVEL)
        > sub_matches.occurrences_of(args::FORMAT)
    {
        error!("Encountered more compression level occurrences than formats");
        return Ok(());
    }

    let input_paths = sub_matches.values_of_t_or_exit::<PathBuf>(args::INPUT_PATHS);
    let mut output_path = sub_matches.value_of_t_or_exit::<PathBuf>(args::OUTPUT_PATH);

    if output_path.is_relative() {
        output_path = Path::new(".").join(output_path);
    }

    if sub_matches.occurrences_of(args::FORMAT_GROUP) > 0 {
        if let Some(values) = sub_matches.grouped_values_of(args::FORMAT_GROUP) {
            // derive archive format and level from arguments
            debug!("Deriving archive format and level from arguments");

            let formats = values
                .map(|vec| vec.first().copied())
                .tuples()
                .map(|(format, level)| {
                    Result::<(ArchiveFormat, CompressionLevel)>::Ok((
                        format
                            .map(ArchiveFormat::from_str)
                            .transpose()?
                            .ok_or(Error::MissingCompressionFormat)?,
                        level
                            .map(CompressionLevel::from_str)
                            .transpose()?
                            .unwrap_or_default(),
                    ))
                })
                .collect::<Result<Vec<_>>>()?;

            do_pack(input_paths, formats, output_path)?;
        } else {
            unreachable!("");
        }
    } else {
        // derive archive format from output file name
        debug!("Deriving archive format from output file name");

        let path = sub_matches.value_of_t_or_exit::<PathBuf>(args::OUTPUT_PATH);

        let formats = path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .split('.')
            .flat_map(ArchiveFormat::try_from_ext)
            .map(|format| (format, CompressionLevel::Auto))
            .collect_vec();

        do_pack(input_paths, formats, output_path)?;
    }

    Ok(())
}

fn do_pack(
    input_paths: Vec<PathBuf>,
    formats: Vec<(ArchiveFormat, CompressionLevel)>,
    output_path: PathBuf,
) -> Result<()> {
    let mut iter = formats.into_iter();
    let mut target_name = output_path
        .file_name()
        .and_then(|s| {
            s.to_string_lossy()
                .split_once('.')
                .map(|(left, _)| left.to_string())
        })
        .unwrap_or_else(|| String::from("archive"));

    trace!("Archive name: {target_name}");

    if let Some((format, level)) = iter.next() {
        debug!("Packing inner most archive");

        target_name.push('.');
        target_name.push_str(format.as_ext());

        pack_files(&input_paths, format, level, &target_name)?;
    }

    for (format, level) in iter {
        let new_target_name = format!("{target_name}.{}", format.as_ext());

        pack_files(
            &[PathBuf::from(&target_name)],
            format,
            level,
            &new_target_name,
        )?;

        fs::remove_file(&target_name).map_err(|source| Error::RemoveOldArchive {
            source,
            path: target_name.to_string(),
        })?;

        target_name = new_target_name;
    }

    Ok(())
}

fn pack_files<OutputPath>(
    paths: &[PathBuf],
    format: ArchiveFormat,
    level: CompressionLevel,
    output_path: OutputPath,
) -> Result<()>
where
    OutputPath: AsRef<Path>,
{
    trace!("Creating archive file {}", output_path.as_ref().display());

    let file = File::create(&output_path).map_err(|source| Error::CreateArchiveFile {
        source,
        path: output_path.as_ref().display().to_string(),
    })?;

    info!(
        "Packing {:?} as {format} with compression level {level} to {}",
        paths,
        output_path.as_ref().display()
    );

    let mut packer = match format {
        ArchiveFormat::Tar => TarPacker::new(&file)?,
        ArchiveFormat::Zip => todo!(),
        ArchiveFormat::GZip => todo!(),
        ArchiveFormat::Lzma => todo!(),
    };

    for path in paths {
        packer.add_path(path)?;
    }

    packer.finish()?;

    Ok(())
}
