#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]

use std::borrow::Cow;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct RonConfig {
    pub angelface: char,
    pub countdown: Cow<'static, [i64]>,
    pub empty: Cow<'static, [()]>,
    pub float: f64,
    pub integer: i64,
    pub is_true: bool,
    pub name: Cow<'static, str>,
    pub nothing: Option<()>,
    pub objects: Cow<'static, [_Config__objects]>,
    pub something: Option<i64>,
    pub structure: _Config__structure,
    pub unit: (),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__objects {
    pub index: i64,
    pub name: Cow<'static, str>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct _Config__structure {
    pub name: Cow<'static, str>,
    pub status: Cow<'static, str>,
}

pub const RONCONFIG: RonConfig = RonConfig {
    angelface: 'A',
    countdown: Cow::Borrowed(&[3, 2, 1]),
    empty: Cow::Borrowed(&[]),
    float: 100.1,
    integer: 100,
    is_true: true,
    name: Cow::Borrowed("Config name"),
    nothing: None,
    objects: Cow::Borrowed(&[_Config__objects {
            index: 0,
            name: Cow::Borrowed("Thing 1"),
        }, _Config__objects {
            index: 1,
            name: Cow::Borrowed("Thing 2"),
        }]),
    something: Some(10),
    structure: _Config__structure {
        name: Cow::Borrowed("Doesn't have one, sadly."),
        status: Cow::Borrowed("Naw too bad."),
    },
    unit: (),
};

#[cfg(debug_assertions)]
impl RonConfig {
    pub fn load() -> Cow<'static, Self> {
        let filepath = concat!(env!("CARGO_MANIFEST_DIR"), "/config.ron");
        Self::load_from(filepath.as_ref()).expect("Failed to load RonConfig.")
    }

    pub fn load_from(filepath: &::std::path::Path) -> Result<Cow<'static, Self>, Box<dyn ::std::error::Error>> {
        let file_contents = ::std::fs::read_to_string(filepath)?;
        let result: Self = ::ron::de::from_str(&file_contents)?;
        Ok(Cow::Owned(result))
    }
}

#[cfg(not(debug_assertions))]
impl RonConfig {
    #[inline(always)]
    pub fn load() -> Cow<'static, Self> {
        Cow::Borrowed(&RONCONFIG)
    }

    #[inline(always)]
    pub fn load_from(_: &::std::path::Path) -> Result<Cow<'static, Self>, Box<dyn ::std::error::Error>> {
        Ok(Cow::Borrowed(&RONCONFIG))
    }
}
