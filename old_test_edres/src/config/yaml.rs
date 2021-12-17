#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]

use std::borrow::Cow;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct YamlConfig {
    pub array_of_structs: Cow<'static, [_Config__array_of_structs]>,
    pub coord: Cow<'static, [f64]>,
    pub empty: Cow<'static, [()]>,
    pub floaty: f64,
    pub i64_max: i64,
    pub is_config: bool,
    pub is_not_config: bool,
    pub name: Cow<'static, str>,
    pub nested: _Config__nested,
    pub nothing: Option<()>,
    pub number: i64,
    pub u64_max: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__array_of_structs {
    pub n: i64,
    pub name: Cow<'static, str>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__nested {
    pub name: Cow<'static, str>,
    pub values: _Config__nested__values,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__nested__values {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

pub const YAML_CONFIG: YamlConfig = YamlConfig {
    array_of_structs: Cow::Borrowed(&[_Config__array_of_structs {
            n: 0,
            name: Cow::Borrowed("first"),
        }, _Config__array_of_structs {
            n: 1,
            name: Cow::Borrowed("second"),
        }]),
    coord: Cow::Borrowed(&[-5.0, 5.0]),
    empty: Cow::Borrowed(&[]),
    floaty: 123.456789,
    i64_max: 9223372036854775807,
    is_config: true,
    is_not_config: false,
    name: Cow::Borrowed("Config name"),
    nested: _Config__nested {
        name: Cow::Borrowed("nested2"),
        values: _Config__nested__values {
            x: 0,
            y: 1,
            z: 2,
        },
    },
    nothing: None,
    number: 100,
    u64_max: 18446744073709551615,
};

#[cfg(debug_assertions)]
impl YamlConfig {
    pub fn load() -> Cow<'static, Self> {
        let filepath = concat!(env!("CARGO_MANIFEST_DIR"), "/config.yaml");
        Self::load_from(filepath.as_ref()).expect("Failed to load YamlConfig.")
    }

    pub fn load_from(filepath: &::std::path::Path) -> Result<Cow<'static, Self>, Box<dyn ::std::error::Error>> {
        let file_contents = ::std::fs::read_to_string(filepath)?;
        let result: Self = ::serde_yaml::from_str(&file_contents)?;
        Ok(Cow::Owned(result))
    }
}

#[cfg(not(debug_assertions))]
impl YamlConfig {
    #[inline(always)]
    pub fn load() -> Cow<'static, Self> {
        Cow::Borrowed(&YAML_CONFIG)
    }

    #[inline(always)]
    pub fn load_from(_: &::std::path::Path) -> Result<Cow<'static, Self>, Box<dyn ::std::error::Error>> {
        Ok(Cow::Borrowed(&YAML_CONFIG))
    }
}
