use std::path::PathBuf;

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Create nix builds from Arduino projects.
pub struct Arguments {
    #[argh(option)]
    /// indicate the root of the project directory
    pub project_root: Option<PathBuf>,

    #[argh(subcommand)]
    pub command: Command,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Command {
    Generate(Generate),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Generate Arduino.nix
#[argh(subcommand, name = "generate")]
pub struct Generate {
    #[argh(option)]
    /// override the output file path, use - to indicate stdout
    pub output_file: Option<PathBuf>,
}
