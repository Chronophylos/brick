use std::{path::PathBuf, str::FromStr};

use clap::{command, Arg, ArgGroup, ArgMatches, Command};
use itertools::Itertools;
use log::{debug, error, info, LevelFilter};

use brick::{
    error::{Error, Result},
    ArchiveFormat, CompressionLevel,
};

static COMPRESSION_LEVEL_ARG: &str = "compression level";
static FORMAT_ARG: &str = "format";
static FORMAT_ARG_GROUP: &str = "format group";
static INPUT_FILES_ARG: &str = "input files";
static OUTPUT_FILE_ARG: &str = "ouput";
static LOG_LEVEL_ARG_GROUP: &str = "log level group";

fn main() -> Result<()> {
    let app = create_app();

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

fn pack_command() -> Command<'static> {
    Command::new("pack")
        .visible_alias("p")
        .about("Pack files and directories into an archive")
        .arg_required_else_help(true)
        .group(
            ArgGroup::new(FORMAT_ARG_GROUP)
                .multiple(true)
                .args(&[FORMAT_ARG, COMPRESSION_LEVEL_ARG]),
        )
        .args(&[
            Arg::new(FORMAT_ARG)
                .short('f')
                .long("format")
                .help("Specify the compression format")
                .possible_values(ArchiveFormat::all())
                .multiple_occurrences(true)
                .takes_value(true),
            Arg::new(COMPRESSION_LEVEL_ARG)
                .short('c')
                .long("compression")
                .help("Specify the compresion level [possible values: auto, 0-9]")
                .long_help(
                    "Specify the compression level from 0..9.
Default if no value specified or not set is auto. 
auto means compression level is decided bytodo!() the format

Example:
`-c` -> auto
`-c 0` -> 0 
",
                )
                .value_name("level")
                .requires(FORMAT_ARG)
                .multiple_occurrences(true)
                .takes_value(true)
                .default_value("auto")
                .hide_possible_values(true)
                .possible_values(["auto", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"])
                .ignore_case(true),
            Arg::new(INPUT_FILES_ARG)
                .help("files and directories to pack")
                .required(true)
                .takes_value(true)
                .multiple_values(true)
                .value_name("INPUT"),
            Arg::new(OUTPUT_FILE_ARG)
                .help("output file")
                .last(true)
                .required_unless_present_any(&[FORMAT_ARG_GROUP]),
        ])
}

fn create_app() -> Command<'static> {
    command!()
        .subcommand_required(true)
        .help_expected(true)
        .args(&[
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("reduce output. Multiple occurences make output less informative")
                .long_help(
"Decrease the logging level. The default level is info. Each occurrance decreases the level from info to warn to error to nothing"
                )
                .group(LOG_LEVEL_ARG_GROUP)
                .max_occurrences(3)
                .multiple_occurrences(true),
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("explain what is beeing done. Multiple occurences make output more informative")
                .long_help(
"Increase the logging level. The default level is info. Each occurrance increases the level from info to debug to trace"
                )
                .group(LOG_LEVEL_ARG_GROUP)
                .max_occurrences(2)
                .multiple_occurrences(true),
        ])
        .subcommands(vec![
            Command::new("info")
                .visible_alias("i")
                .about("Display info on an archive")
                .arg_required_else_help(true),
            pack_command(),
            Command::new("unpack")
                .visible_alias("u")
                .about("Unpack an archive")
                .arg_required_else_help(true),
        ])
}

fn pack(sub_matches: &ArgMatches) -> Result<()> {
    if sub_matches.occurrences_of(COMPRESSION_LEVEL_ARG) > sub_matches.occurrences_of(FORMAT_ARG) {
        error!("Encountered more compression level occurrences than formats");
        return Ok(());
    }

    if sub_matches.occurrences_of(FORMAT_ARG_GROUP) > 0 {
        if let Some(values) = sub_matches.grouped_values_of(FORMAT_ARG_GROUP) {
            // derive archive format and level from arguments
            debug!("Deriving archive format and level from arguments");

            for (format, level) in values
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
                .collect::<Result<Vec<_>>>()?
            {
                info!("Packing as {format} with compression level {level}")
            }
        } else {
            unreachable!("");
        }
    } else {
        // derive archive format from output file name
        debug!("Deriving archive format from output file name");

        let path = sub_matches.value_of_t_or_exit::<PathBuf>(OUTPUT_FILE_ARG);
        let level = CompressionLevel::Auto;

        for format in path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .split('.')
            .flat_map(ArchiveFormat::try_from_ext)
        {
            info!("Packing as {format} with compression level {level}")
        }
    }

    Ok(())
}
