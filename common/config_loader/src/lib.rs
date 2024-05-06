use std::env;
use std::path::PathBuf;

use bevy::prelude::Resource;
pub use bevy_persistent::Persistent;
use bevy_persistent::StorageFormat;
use serde::de::DeserializeOwned;
use serde::Serialize;

const CONFIG_DIR_NAME: &str = "config";
const CONFIG_FILE_EXTENSION: &str = ".toml";

// todo tests, watching (hot reload) [https://github.com/umut-sahin/bevy-persistent/issues/39]
// todo remove panics
pub fn load<T: Config>() -> Persistent<T> {
    let config_dir = config_dir_path();
    let file_name = format!("{}{}", to_formatted_path(&T::name()), CONFIG_FILE_EXTENSION);
    let config_file_path = config_dir.join(file_name);

    Persistent::<T>::builder()
        .name(T::name())
        .format(StorageFormat::TomlPretty)
        .path(config_file_path)
        .default(Default::default())
        .build()
        .expect("failed to initialize config")
}

// todo derive config with optional name parameter
pub trait Config: Resource + Serialize + DeserializeOwned + Default + Send + Sync + 'static {
    fn name() -> String {
        std::any::type_name::<Self>().to_string()
    }
}

// todo remove panic
fn config_dir_path() -> PathBuf {
    let mut config_dir = env::current_exe().unwrap();
    config_dir.pop();
    config_dir.push(CONFIG_DIR_NAME);
    config_dir
}

fn to_formatted_path(value: &str) -> String {
    let mut formatted_value = value.replace("::", ".");
    formatted_value = formatted_value.replace('<', "[");
    formatted_value = formatted_value.replace('>', "]");
    formatted_value
}
