use std::env;
use clap::{Arg, Command};

pub const PROGRAM_NAME: &str = "list";

pub fn parse_cli_arguments() -> Command {
    Command::new(PROGRAM_NAME)
        .version(env!("CARGO_PKG_VERSION")) // set version from Cargo.toml
        .about("List files")
        .long_about(format!(
            "List files.\n\n\
            Example usage:\n    {} <path>",
            PROGRAM_NAME))
        .arg(Arg::new("path")
            .help("path")
            .index(1))
        .arg(Arg::new("max")
                .short('m')
                .long("max")
                .value_parser(clap::value_parser!(usize))
                .help("Maximum number of directory levels displayed"))
}
