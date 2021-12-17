#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]

use std::borrow::Cow;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct StaticConfig {
    pub name: Cow<'static, str>,
}

pub const STATIC_CONFIG: StaticConfig = StaticConfig {
    name: Cow::Borrowed("Example Config"),
};
impl StaticConfig {
    #[inline(always)]
    pub fn load() -> Cow<'static, Self> {
        Cow::Borrowed(&STATIC_CONFIG)
    }

    #[inline(always)]
    pub fn load_from(_: &::std::path::Path) -> Result<Cow<'static, Self>, Box<dyn ::std::error::Error>> {
        Ok(Cow::Borrowed(&STATIC_CONFIG))
    }
}