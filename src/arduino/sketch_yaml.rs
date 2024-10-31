use std::collections::HashMap;

use nom::{
    bytes::complete::{take_while, take_while1},
    character::complete::{char as nom_char, digit1},
    combinator::{map, map_res},
    sequence::{delimited, pair, separated_pair},
};
use semver::Version;
use serde::{de::Visitor, Deserialize, Serialize};
use serde_yaml_ng::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub fqbn: String,

    #[serde(default)]
    pub platforms: Vec<Platform>,

    #[serde(default)]
    // TODO these require parsing similar to platform names.
    pub libraries: Vec<String>,

    /// When re-serializing, we want to restore all values we didn't know what to do with.
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Platform {
    pub platform: PlatformName,
    pub platform_index_url: String,

    /// When re-serializing, we want to restore all values we didn't know what to do with.
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PlatformName {
    pub package: String,
    pub platform: String,
    pub version: Version,
}

impl PlatformName {
    fn space_char(c: char) -> bool {
        " \t".contains(c)
    }

    fn name_char(c: char) -> bool {
        c.is_alphanumeric() | "_-".contains(c)
    }

    fn space(input: &str) -> nom::IResult<&str, &str> {
        take_while(Self::space_char)(input)
    }

    fn name(input: &str) -> nom::IResult<&str, &str> {
        take_while1(Self::name_char)(input)
    }

    fn number(input: &str) -> nom::IResult<&str, u64> {
        map_res(digit1, |s: &str| s.parse())(input)
    }

    fn version(input: &str) -> nom::IResult<&str, Version> {
        map(
            separated_pair(
                Self::number,
                nom_char('.'),
                separated_pair(Self::number, nom_char('.'), Self::number),
            ),
            |(major, (minor, patch))| Version::new(major, minor, patch),
        )(input)
    }

    fn parse(input: &str) -> nom::IResult<&str, Self> {
        map(
            separated_pair(
                separated_pair(
                    Self::name,
                    separated_pair(Self::space, nom_char(':'), Self::space),
                    Self::name,
                ),
                Self::space,
                delimited(
                    pair(nom_char('('), Self::space),
                    Self::version,
                    pair(Self::space, nom_char(')')),
                ),
            ),
            |((package, platform), version)| Self {
                package: package.to_string(),
                platform: platform.to_string(),
                version,
            },
        )(input)
    }
}

impl<'de> Deserialize<'de> for PlatformName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PlatformVisitor;

        impl<'de> Visitor<'de> for PlatformVisitor {
            type Value = PlatformName;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(
                    "The platform identifier and version number `package:platform (1.2.3)",
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                PlatformName::parse(v)
                    .map(|(_remaining, name)| name)
                    .map_err(|error| E::custom(error))
            }
        }

        deserializer.deserialize_str(PlatformVisitor)
    }
}

impl Serialize for PlatformName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!(
            "{}:{} ({})",
            self.package, self.platform, self.version
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_platform_name_and_version() {
        assert_eq!(
            PlatformName::parse("my_package:my_platform(1.2.3)"),
            Ok((
                "",
                PlatformName {
                    package: "my_package".into(),
                    platform: "my_platform".into(),
                    version: Version::new(1, 2, 3)
                }
            ))
        );
    }
}
