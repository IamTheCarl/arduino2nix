use std::path::PathBuf;

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Create nix builds from Arduino projects.
struct Arguments {
    #[argh(option)]
    /// indicate the root of the project directory
    project_root: Option<PathBuf>,

    #[argh(subcommand)]
    command: Command,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Command {
    Generate(Generate),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Generate Arduino.nix
#[argh(subcommand, name = "generate")]
struct Generate {}
