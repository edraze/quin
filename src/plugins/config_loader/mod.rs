use std::env;
use std::path::PathBuf;

use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{Commands, Resource};
use bevy_persistent::{Persistent, StorageFormat};
use serde::de::DeserializeOwned;
use serde::Serialize;

const CONFIG_DIR_NAME: &str = "config";
const CONFIG_FILE_EXTENSION: &str = ".yml";

// todo tests, watching (hot reload)
#[derive(Default)]
pub struct ConfigLoaderPlugin<T>
    where T: Resource + Serialize + DeserializeOwned + Default + Config + Send + Sync + 'static {
    _config: T,
}

impl<T> Plugin for ConfigLoaderPlugin<T> where
    T: Resource + Serialize + DeserializeOwned + Default + Config + Send + Sync + 'static {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup::<T>);
    }
}

fn setup<T>(mut commands: Commands)
    where T: Resource + Serialize + DeserializeOwned + Default + Config {
    let config_dir = config_dir_path();
    let file_name = format!("{}{}", to_formatted_path(&T::name()), CONFIG_FILE_EXTENSION);
    let config_file_path = config_dir.join(file_name);
    commands.insert_resource(
        Persistent::<T>::builder()
            .name(T::name())
            .format(StorageFormat::Yaml)
            .path(config_file_path)
            .default(Default::default())
            .build()
            .expect("failed to initialize config")
    )
}

// todo derive
pub trait Config {
    fn name() -> String {
        std::any::type_name::<Self>().to_string()
    }
}

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
