mod cli;

use std::{path::PathBuf, str::FromStr};

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

use brick::{
    error::{Error, Result},
    ArchiveFormat, CompressionLevel,
};

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}
pub(crate) use fl;

pub(crate) static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader = fluent_language_loader!();
    let requested_languages = DesktopLanguageRequester::requested_languages();
    let _result = i18n_embed::select(&loader, &Localizations, &requested_languages);
    loader
});

fn main() -> Result<()> {
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

            do_pack(formats)?;
        } else {
            unreachable!("");
        }
    } else {
        // derive archive format from output file name
        debug!("Deriving archive format from output file name");

        let path = sub_matches.value_of_t_or_exit::<PathBuf>(args::OUTPUT_FILE);

        let formats = path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .split('.')
            .flat_map(ArchiveFormat::try_from_ext)
            .map(|format| (format, CompressionLevel::Auto))
            .collect_vec();

        do_pack(formats)?;
    }

    Ok(())
}

fn do_pack(formats: Vec<(ArchiveFormat, CompressionLevel)>) -> Result<()> {
    for (format, level) in formats {
        info!("Packing as {format} with compression level {level}")
    }

    Ok(())
}
