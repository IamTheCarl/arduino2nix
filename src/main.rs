use std::{
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{Context, Result};
use arduino::sketch_yaml;
use arguments::{Arguments, Generate};

mod arduino;
mod arguments;
mod generate;

fn main() {
    let mut arguments: Arguments = argh::from_env();
    colog::init();

    let project_root = arguments
        .project_root
        .take()
        .map(Ok)
        .unwrap_or_else(|| std::env::current_dir());

    match project_root {
        Ok(project_root) => {
            let result = match arguments.command {
                arguments::Command::Generate(command) => generate_command(project_root, command),
            };

            if let Err(error) = result {
                log::error!("Failure: {error:?}");
            }
        }
        Err(error) => {
            log::error!("Failed to get project root: {error}");
        }
    }
}

fn generate_command(project_root: PathBuf, command: Generate) -> Result<()> {
    let sketch_yaml =
        File::open(project_root.join("sketch.yaml")).context("Failed to open sketch.yaml")?;
    let sketch_yaml: sketch_yaml::Project =
        serde_yaml_ng::from_reader(sketch_yaml).context("Failed to parse sketch.yaml")?;

    let mut output: Box<dyn io::Write> = if let Some(output_file) = command.output_file {
        if output_file == Path::new("-") {
            Box::new(io::stdout())
        } else {
            let error_message = format!("Failed to create {output_file:?}");

            Box::new(File::create(output_file).context(error_message)?)
        }
    } else {
        Box::new(File::create("Arduino.nix").context("Failed to create Arduino.nix")?)
    };

    let mut body = String::new();

    generate::body(&mut body, sketch_yaml).context("Failed to generate Nix script")?;

    let mut nixfmt = Command::new("nixfmt");
    nixfmt.stdin(Stdio::piped());
    nixfmt.stdout(Stdio::piped());
    nixfmt.stderr(Stdio::inherit());

    let mut nixfmt = nixfmt.spawn().context("Failed to spawn nixfmt")?;

    let mut stdin = nixfmt.stdin.take().unwrap();
    stdin
        .write_all(body.as_bytes())
        .context("Failed to send generated code to nixfmt")?;
    drop(stdin); // Sends EOF to nixfmt.

    let mut stdout = nixfmt.stdout.take().unwrap();
    std::io::copy(&mut stdout, &mut output).context("Failed to copy output of nixfmt to output")?;

    Ok(())
}
