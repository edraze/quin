use std::{env, fs};
use std::io::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

pub use applications_config::KOMOREBI_APPLICATIONS_CONFIG;
pub use config::KOMOREBI_CONFIG;

mod applications_config;
mod config;

const BINARY_PATH: &str = "./bin/komorebi.exe";

pub fn run(config_path: Option<String>) -> Result<(), Error> {
    let binary_path = executable_path().join("bin/komorebi.exe");
    load_binary(&binary_path);
    let mut command = Command::new(BINARY_PATH);

    if let Some(config_path) = config_path {
        command.args(["-c", &config_path]);
    }

    command
        .output()
        .map(|output|
            println!("Komorebi process status: {}, output: {}",
                     output.status, String::from_utf8(output.stdout).unwrap())) // todo remove unwrap
}

// todo remove panics
fn load_binary(path: &Path) {
    if fs::metadata(path).is_err() {
        println!("Komorebi binary missing, try to write binary in path: {path:?}");
        let binary = include_bytes!("../../../bin/komorebi.exe");
        fs::create_dir_all(&path.parent().unwrap()).unwrap();
        fs::write(path, binary).expect("Error during komorebi binary creation");
    }
}

fn executable_path() -> PathBuf {
    let mut config_dir = env::current_exe().expect("Can't get executable path");
    config_dir.pop();
    config_dir
}
