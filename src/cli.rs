use brick::ArchiveFormat;
use clap::{command, Arg, ArgGroup, Command};
use once_cell::sync::Lazy;

use crate::macros::fl;

pub mod args {
    pub static COMPRESSION_LEVEL: &str = "compression level";
    pub static FORMAT: &str = "format";
    pub static FORMAT_GROUP: &str = "format group";
    pub static INPUT_PATHS: &str = "input paths";
    pub static OUTPUT_PATH: &str = "output path";
    pub static LOG_LEVEL_GROUP: &str = "log level group";
}

pub fn app() -> Command<'static> {
    static ABOUT: Lazy<String> = Lazy::new(|| fl!("cli-about"));
    static QUIET_HELP: Lazy<String> = Lazy::new(|| fl!("cli-quiet-help"));
    static QUIET_LONG_HELP: Lazy<String> = Lazy::new(|| fl!("cli-quiet-long-help"));
    static VERBOSE_HELP: Lazy<String> = Lazy::new(|| fl!("cli-verbose-help"));
    static VERBOSE_LONG_HELP: Lazy<String> = Lazy::new(|| fl!("cli-verbose-long-help"));
    static INFO_ABOUT: Lazy<String> = Lazy::new(|| fl!("cli-info-about"));
    static UNPACK_ABOUT: Lazy<String> = Lazy::new(|| fl!("cli-unpack-about"));

    command!()
        .about(ABOUT.as_str())
        .subcommand_required(true)
        .help_expected(true)
        .args(&[
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help(QUIET_HELP.as_str())
                .long_help(QUIET_LONG_HELP.as_str())
                .group(args::LOG_LEVEL_GROUP)
                .max_occurrences(3)
                .multiple_occurrences(true),
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help(VERBOSE_HELP.as_str())
                .long_help(VERBOSE_LONG_HELP.as_str())
                .group(args::LOG_LEVEL_GROUP)
                .max_occurrences(2)
                .multiple_occurrences(true),
        ])
        .subcommands(vec![
            Command::new("info")
                .visible_alias("i")
                .about(INFO_ABOUT.as_str())
                .arg_required_else_help(true),
            pack(),
            Command::new("unpack")
                .visible_alias("u")
                .about(UNPACK_ABOUT.as_str())
                .arg_required_else_help(true),
        ])
}

fn pack() -> Command<'static> {
    static ABOUT: Lazy<String> = Lazy::new(|| fl!("cli-pack-about"));
    static FORMAT_HELP: Lazy<String> = Lazy::new(|| fl!("cli-pack-format-help"));
    static COMPRESSION_HELP: Lazy<String> = Lazy::new(|| fl!("cli-pack-compression-help"));
    static COMPRESSION_LONG_HELP: Lazy<String> =
        Lazy::new(|| fl!("cli-pack-compression-long-help"));
    static COMPRESSION_VALUE_NAME: Lazy<String> =
        Lazy::new(|| fl!("cli-pack-compression-value-name"));
    static INPUT_HELP: Lazy<String> = Lazy::new(|| fl!("cli-pack-input-help"));
    static OUTPUT_HELP: Lazy<String> = Lazy::new(|| fl!("cli-pack-output-help"));

    Command::new("pack")
        .visible_alias("p")
        .about(ABOUT.as_str())
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
                .help(FORMAT_HELP.as_str())
                .possible_values(ArchiveFormat::all())
                .multiple_occurrences(true)
                .takes_value(true),
            Arg::new(args::COMPRESSION_LEVEL)
                .short('c')
                .long("compression")
                .help(COMPRESSION_HELP.as_str())
                .long_help(COMPRESSION_LONG_HELP.as_str())
                .value_name(COMPRESSION_VALUE_NAME.as_str())
                .requires(args::FORMAT)
                .multiple_occurrences(true)
                .takes_value(true)
                .default_value("auto")
                .hide_possible_values(true)
                .possible_values(["auto", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"])
                .ignore_case(true),
            Arg::new(args::INPUT_PATHS)
                .help(INPUT_HELP.as_str())
                .required(true)
                .takes_value(true)
                .multiple_values(true)
                .value_name("INPUT"),
            Arg::new(args::OUTPUT_PATH)
                .help(OUTPUT_HELP.as_str())
                .last(true)
                .required_unless_present_any(&[args::FORMAT_GROUP]),
        ])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_app() {
        app().debug_assert();
    }
}
