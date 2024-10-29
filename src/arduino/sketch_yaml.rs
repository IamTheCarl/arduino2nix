use std::collections::HashMap;

use nom::{
    bytes::complete::{take_while, take_while1},
    character::complete::{char as nom_char, digit1},
    combinator::{map, map_res},
    sequence::{delimited, pair, separated_pair},
};
use semver::Version;
use serde::{de::Visitor, Deserialize};
use url::Url;

#[derive(Deserialize)]
pub struct Project {
    pub profiles: HashMap<String, Project>,
}

#[derive(Deserialize)]
pub struct Profile {
    pub fqbn: String,

    #[serde(default)]
    pub platforms: Vec<Platform>,

    #[serde(default)]
    pub libraries: HashMap<String, Version>,
}

#[derive(Deserialize)]
pub struct Platform {
    pub platform: String, // <PLATFORM> (<PLATFORM_VERSION>)
    pub platform_index_url: Url,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PlatformName {
    pub name: String,
    pub version: Version,
}

impl PlatformName {
    fn space_char(c: char) -> bool {
        " \t".contains(c)
    }

    fn name_char(c: char) -> bool {
        c.is_alphanumeric() | "_-:".contains(c)
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
                Self::name,
                Self::space,
                delimited(
                    pair(nom_char('('), Self::space),
                    Self::version,
                    pair(Self::space, nom_char(')')),
                ),
            ),
            |(name, version)| Self {
                name: name.to_string(),
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
                formatter.write_str("The platform name and version number `name (1.2.3)")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                todo!()
            }
        }

        deserializer.deserialize_str(PlatformVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_platform_name_and_version() {
        assert_eq!(
            PlatformName::parse("my_platform(1.2.3)"),
            Ok((
                "",
                PlatformName {
                    name: "my_platform".into(),
                    version: Version::new(1, 2, 3)
                }
            ))
        );
        assert_eq!(
            PlatformName::parse("my_platform (1.2.3)"),
            Ok((
                "",
                PlatformName {
                    name: "my_platform".into(),
                    version: Version::new(1, 2, 3)
                }
            ))
        );
        assert_eq!(
            PlatformName::parse("my_platform\t( 1.2.3 )"),
            Ok((
                "",
                PlatformName {
                    name: "my_platform".into(),
                    version: Version::new(1, 2, 3)
                }
            ))
        );
    }
}
