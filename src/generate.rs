use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
};

use crate::arduino::{
    index_json::IndexFile,
    sketch_yaml::{Profile, Project},
};

/// Produces a Nix script that will provide all of the platforms to arduino-cli.
/// It returns new platforms to be inserted into sketch.yaml. They will reference
/// the Nixified platforms.
pub fn body<'p>(output: &mut impl Write, mut project: Project) -> Result<()> {
    writeln!(output, "# Auto-generated with arduino2nix. Do not modify.")?;
    writeln!(
        output,
        "# Edit your sketch.yaml and then run `arduino2nix generate` to update this file."
    )?;
    writeln!(output, "{{ pkgs ? <nixpkgs> {{}}, pname, version }}:")?;
    writeln!(output, "let")?;

    generate_profiles(output, &mut project.profiles)?;
    generate_sketch_yaml(output, &project)?;

    writeln!(output, "in")?;

    writeln!(output, "pkgs.stdenv.mkDerivation {{")?;

    writeln!(output, "inherit pname version;")?;
    writeln!(output, "src = ./.;")?;

    writeln!(output, "buildInputs = [ pkgs.arduino-cli ];")?;

    writeln!(
        output,
        "postUnpack = ''
        rm -f $sourceRoot/sketch.yaml
        ln -s ${{sketch_yaml}} $sourceRoot/sketch.yaml
        cat $sourceRoot/sketch.yaml
        '';"
    )?;
    writeln!(
        output,
        "buildPhase = ''
        arduino-cli compile --profile ${{pname}} --output-dir output
        '';"
    )?;
    writeln!(
        output,
        "installPhase = ''
        cp output/${{pname}}.ino.elf $out/payload.elf
        '';"
    )?;

    writeln!(output, "}}")?;

    Ok(())
}

/// Generates profile derivations and updates the original profile configurations to point to the new sources.
fn generate_profiles(
    output: &mut impl Write,
    profiles: &mut HashMap<String, Profile>,
) -> Result<()> {
    let mut platforms = HashSet::new();

    // We start by accumulating platforms.
    for profile in profiles.values_mut() {
        for platform in profile.platforms.iter_mut() {
            let variable_name = format!(
                "{}-{}-v{}",
                platform.platform.package.to_case(Case::Kebab),
                platform.platform.platform.to_case(Case::Kebab),
                platform.platform.version.to_string().replace(".", "-")
            );

            if !platforms.contains(&variable_name) {
                let index = ureq::request("GET", &platform.platform_index_url)
                    .call()
                    .with_context(|| {
                        format!(
                            "Failed to download platform {variable_name} from {}",
                            platform.platform_index_url
                        )
                    })?;

                let index: IndexFile =
                    serde_json::from_reader(index.into_reader()).context("Failed to parse Json")?;

                let package = index
                    .packages
                    .iter()
                    .find(|package| package.name == platform.platform.platform)
                    .context("Failed to find platform package in platform index")?;

                let index_platform = package
                    .platforms
                    .iter()
                    .find(|index_platform| index_platform.name == platform.platform.platform)
                    .context("Failed to find platform in package")?;

                writeln!(
                    output,
                    "{variable_name} = (pkgs.fetchurl {{ url = \"{}\"; sha256 = \"{}\"; }});",
                    index_platform.url,
                    index_platform.checksum.replace("SHA-256:", "")
                )?;

                // That structure is going to be re-serialized back into a yaml file. We need it to point to
                // the new location of the platform index file, within the nix store.
                platform.platform_index_url = format!("file://${{{variable_name}}}");

                platforms.insert(variable_name);
            }
        }
    }

    Ok(())
}

fn generate_sketch_yaml(output: &mut impl Write, project: &Project) -> Result<()> {
    let project_yaml =
        serde_yaml_ng::to_string(project).context("Failed to re-serialize sketch.yaml")?;
    writeln!(
        output,
        "sketch_yaml = pkgs.writeText \"sketch.json\" ''\n{project_yaml}\n'';"
    )?;

    Ok(())
}
