use brick::ArchiveFormat;
use clap::{command, Arg, ArgGroup, Command};

pub mod args {
    pub static COMPRESSION_LEVEL: &str = "compression level";
    pub static FORMAT: &str = "format";
    pub static FORMAT_GROUP: &str = "format group";
    pub static INPUT_FILES: &str = "input files";
    pub static OUTPUT_FILE: &str = "ouput";
    pub static LOG_LEVEL_GROUP: &str = "log level group";
}

pub fn app() -> Command<'static> {
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
                .group(args::LOG_LEVEL_GROUP)
                .max_occurrences(3)
                .multiple_occurrences(true),
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("explain what is beeing done. Multiple occurences make output more informative")
                .long_help(
"Increase the logging level. The default level is info. Each occurrance increases the level from info to debug to trace"
                )
                .group(args::LOG_LEVEL_GROUP)
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

fn pack_command() -> Command<'static> {
    Command::new("pack")
        .visible_alias("p")
        .about("Pack files and directories into an archive")
        .arg_required_else_help(true)
        .group(
            ArgGroup::new(args::FORMAT_GROUP)
                .multiple(true)
                .args(&[args::FORMAT, args::COMPRESSION_LEVEL]),
        )
        .args(&[
            Arg::new(args::FORMAT)
                .short('f')
                .long("format")
                .help("Specify the compression format")
                .possible_values(ArchiveFormat::all())
                .multiple_occurrences(true)
                .takes_value(true),
            Arg::new(args::COMPRESSION_LEVEL)
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
                .requires(args::FORMAT)
                .multiple_occurrences(true)
                .takes_value(true)
                .default_value("auto")
                .hide_possible_values(true)
                .possible_values(["auto", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"])
                .ignore_case(true),
            Arg::new(args::INPUT_FILES)
                .help("files and directories to pack")
                .required(true)
                .takes_value(true)
                .multiple_values(true)
                .value_name("INPUT"),
            Arg::new(args::OUTPUT_FILE)
                .help("output file")
                .last(true)
                .required_unless_present_any(&[args::FORMAT_GROUP]),
        ])
}
