#![feature(io_error_more)]

mod cli;
mod macros;

use std::{
    fs,
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
use log::{debug, error, info, LevelFilter};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use tempfile::NamedTempFile;

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

    println!("Using log level {log_level}");

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

    if let Some((format, level)) = iter.next() {
        let path = pack_files(&input_paths, format, level)?;

        let final_path = iter.try_fold(path, |path, (format, level)| {
            pack_files(&[path], format, level)
        })?;

        info!(
            "Moving {} to {}",
            final_path.display(),
            output_path.display()
        );

        match fs::rename(&final_path, &output_path) {
            Err(err) if err.kind() == std::io::ErrorKind::CrossesDevices => {
                fs::copy(&final_path, &output_path).map(|_| ())
            }
            result => result,
        }
        .map_err(|source| Error::MoveFinalArchive {
            source,
            from: final_path.display().to_string(),
            to: output_path.display().to_string(),
        })?;
    }

    Ok(())
}

fn pack_files(
    paths: &[PathBuf],
    format: ArchiveFormat,
    level: CompressionLevel,
) -> Result<PathBuf> {
    let temp_file = NamedTempFile::new().map_err(Error::CreateTempFile)?;
    let (file, path) = temp_file.keep()?;

    info!(
        "Packing as {format} with compression level {level} to {}",
        path.display()
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

    Ok(path)
}
