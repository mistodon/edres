#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]

use std::borrow::Cow;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct DynamicConfig {
    pub name: Cow<'static, str>,
}

pub const DYNAMIC_CONFIG: DynamicConfig = DynamicConfig {
    name: Cow::Borrowed("Example Config"),
};
impl DynamicConfig {
    pub fn load() -> Cow<'static, Self> {
        let filepath = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/temp/example_config.json");
        Self::load_from(filepath.as_ref()).expect("Failed to load DynamicConfig.")
    }

    pub fn load_from(filepath: &::std::path::Path) -> Result<Cow<'static, Self>, Box<dyn ::std::error::Error>> {
        let file_contents = ::std::fs::read_to_string(filepath)?;
        let result: Self = ::serde_json::from_str(&file_contents)?;
        Ok(Cow::Owned(result))
    }
}