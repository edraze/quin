use std::{fs, thread};
use std::thread::JoinHandle;

use bevy::prelude::Resource;

const KOMOREBI_CONFIG_FOLDER_NAME: &str = "komorebi";

#[derive(Resource)]
pub struct KomorebiState {
    // todo drop thread on plugin reload
    _komorebi_thead: JoinHandle<()>,
}

// todo remove panics
impl KomorebiState {
    pub fn init(komorebi_config: &str, applications_config: &str) -> Self {
        let config_file_path = init_configurations(komorebi_config, applications_config);

        let _komorebi_thead = thread::spawn(move || {
            komorebi::run(Some(&config_file_path))
                .map_err(|error|
                    println!("Failed to run Komorebi in separate thread, executable path: {config_file_path}, error: {error}"))
                .unwrap()
        });
        Self {
            _komorebi_thead,
        }
    }
}

fn init_configurations(komorebi_config: &str, applications_config: &str) -> String {
    let config_folder_path = config_loader::config_dir_path().join(KOMOREBI_CONFIG_FOLDER_NAME);
    if !config_folder_path.exists() {
        fs::create_dir_all(&config_folder_path).unwrap();
    }

    let applications_config_file_path = config_folder_path.join("applications.yml");
    if !applications_config_file_path.exists() {
        fs::write(&applications_config_file_path, applications_config).expect("Error during applications config creation");
    }

    let komorebi_config_file_path = config_folder_path.join("komorebi.json");
    if !komorebi_config_file_path.exists() {
        let komorebi_config = komorebi_config.replace("$APPLICATIONS_CONFIG_PATH",
                                                      &applications_config_file_path.to_string_lossy().to_string()
                                                          .replace("\\", "\\\\"));
        fs::write(&komorebi_config_file_path, komorebi_config).expect("Error during komorebi config creation");
    }
    komorebi_config_file_path.to_string_lossy().to_string()
}
