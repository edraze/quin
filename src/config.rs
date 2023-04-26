use std::fs;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use toml::map::Map;
use toml::Value;

const DEFAULT_CONFIG_PATH: &str = "./config.toml";
const COMMON_ACTIVE_HANDLERS_FIELD: &str = "active_handlers";

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
    pub common: Map<String, Value>,
    #[serde(default)]
    pub handlers: Map<String, Value>,
}

impl Config {
    pub fn is_handler_active(&self, handler_id: &str) -> bool {
        let common_active_handlers_opt = self.common.get(COMMON_ACTIVE_HANDLERS_FIELD);
        if let Some(active_handlers) = common_active_handlers_opt {
            return match active_handlers {
                Value::Array(values) => values.contains(&Value::String(handler_id.into())),
                _ => false
            };
        }
        false
    }

    pub fn get_handler_config<T: Default + DeserializeOwned>(&self, handler_id: &str) -> T {
        self.handlers.get(handler_id)
            .map(|handler_config| toml::from_str(&toml::to_string(handler_config).unwrap()).unwrap())
            .unwrap_or_default()
    }
}
