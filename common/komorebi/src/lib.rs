use std::fs;
use std::io::Error;
use std::process::Command;

const BINARY_PATH: &str = "./bin/komorebi.exe";

pub fn run() -> Result<(), Error> {
    load_binary(BINARY_PATH);
    Command::new(BINARY_PATH)
        .output()
        .map(|output|
            println!("Komorebi process status: {}, output: {}",
                     output.status, String::from_utf8(output.stdout).unwrap())) // todo remove unwrap
}

fn load_binary(path: &str) {
    if fs::metadata(path).is_err() {
        println!("Komorebi binary missing, try to write binary in path: {path}");
        let binary = include_bytes!("../../../bin/komorebi-0.1.26.exe");
        fs::write(path, binary).expect("Error during komorebi binary creation");
    }
}
