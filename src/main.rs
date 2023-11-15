use serde::{Deserialize, Serialize};
use serde_unit_struct::{Deserialize_unit_struct, Serialize_unit_struct};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ConfigToml {
    pub global_index: u64,
    pub key: TomlKey,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TomlKey {
    CaseOne { index: u64, content: Content },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize_unit_struct, Serialize_unit_struct)]
pub struct Content;

fn main() {
    println!("this works with `0.1.1` and fails with `0.1.2`");
    let toml_key = TomlKey::CaseOne {
        index: 42,
        content: Content,
    };
    let config_toml = ConfigToml {
        global_index: 1,
        key: toml_key,
    };
    let config_toml_string = toml::to_string(&config_toml).unwrap();
    let config_toml: ConfigToml = toml::from_str(config_toml_string.as_str()).expect("can roundtrip");
}
