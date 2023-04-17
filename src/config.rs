use std::collections::HashMap;
use std::fs;
use serde::Deserialize;

const DEFAULT_CONFIG_PATH: &str = "./config.toml";

pub struct ConfigLoader {}

impl ConfigLoader {
    fn load_config(path_to_file: &str) -> Config {
        let config_string = fs::read_to_string(path_to_file)
            .expect("Config don't exist. Please create config.toml in root directory"); // todo on missed create config with default bindings
        toml::from_str(&config_string)
            .expect("Can't deserialize config.toml. Looks like it is invalid")
    }

    pub fn load_default() -> Config {
        Self::load_config(DEFAULT_CONFIG_PATH)
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bindings: HashMap<String, String>
}
