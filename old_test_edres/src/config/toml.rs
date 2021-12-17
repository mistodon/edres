#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]

use std::borrow::Cow;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct TomlConfig {
    pub arrayble: Cow<'static, [_Config__arrayble]>,
    pub color: Cow<'static, [i64]>,
    pub coord: Cow<'static, [f64]>,
    pub empty: Cow<'static, [()]>,
    pub floaty: f64,
    pub is_config: bool,
    pub is_not_config: bool,
    pub name: Cow<'static, str>,
    pub number: i64,
    pub one_point_five: f64,
    pub one_point_zero: f64,
    pub points: Cow<'static, [Cow<'static, [i64]>]>,
    pub table: _Config__table,
    pub words: Cow<'static, [Cow<'static, str>]>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__arrayble {
    pub description: Cow<'static, str>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__table {
    pub magnitude: i64,
    pub name: Cow<'static, str>,
    pub table_again: _Config__table__table_again,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__table__table_again {
    pub description: Cow<'static, str>,
    pub name: Cow<'static, str>,
}

pub const TOMLCONFIG: TomlConfig = TomlConfig {
    arrayble: Cow::Borrowed(&[_Config__arrayble {
            description: Cow::Borrowed("just unbelievable"),
        }, _Config__arrayble {
            description: Cow::Borrowed("what is this syntax"),
        }]),
    color: Cow::Borrowed(&[0, 64, 128, 255]),
    coord: Cow::Borrowed(&[-5.0, 5.0]),
    empty: Cow::Borrowed(&[]),
    floaty: 123.456789,
    is_config: true,
    is_not_config: false,
    name: Cow::Borrowed("Config name"),
    number: 100,
    one_point_five: 1.5,
    one_point_zero: 1.0,
    points: Cow::Borrowed(&[Cow::Borrowed(&[1, 2]), Cow::Borrowed(&[3, 4]), Cow::Borrowed(&[5, 6])]),
    table: _Config__table {
        magnitude: 1000000000,
        name: Cow::Borrowed("A table"),
        table_again: _Config__table__table_again {
            description: Cow::Borrowed("getting ridiculous"),
            name: Cow::Borrowed("OK this is just getting ridiculous"),
        },
    },
    words: Cow::Borrowed(&[Cow::Borrowed("one"), Cow::Borrowed("two"), Cow::Borrowed("three")]),
};

#[cfg(debug_assertions)]
impl TomlConfig {
    pub fn load() -> Cow<'static, Self> {
        let filepath = concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml");
        Self::load_from(filepath.as_ref()).expect("Failed to load TomlConfig.")
    }

    pub fn load_from(filepath: &::std::path::Path) -> Result<Cow<'static, Self>, Box<dyn ::std::error::Error>> {
        let file_contents = ::std::fs::read_to_string(filepath)?;
        let result: Self = ::toml::from_str(&file_contents)?;
        Ok(Cow::Owned(result))
    }
}

#[cfg(not(debug_assertions))]
impl TomlConfig {
    #[inline(always)]
    pub fn load() -> Cow<'static, Self> {
        Cow::Borrowed(&TOMLCONFIG)
    }

    #[inline(always)]
    pub fn load_from(_: &::std::path::Path) -> Result<Cow<'static, Self>, Box<dyn ::std::error::Error>> {
        Ok(Cow::Borrowed(&TOMLCONFIG))
    }
}
