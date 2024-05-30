use std::thread;
use std::thread::JoinHandle;

use bevy::prelude::Resource;

const KOMOREBI_CLIENT_NAME: &str = "quin.sock";

#[derive(Resource)]
pub struct KomorebiState {
    // todo drop thread on plugin reload
    _komorebi_thead: JoinHandle<()>,
}

// todo remove panics
impl Default for KomorebiState {
    fn default() -> Self {
        let _komorebi_thead = thread::spawn(move || {
            komorebi::run()
                .map_err(|error|
                    println!("Failed to run Komorebi in separate thread, error: {error}"))
                .unwrap()
        });
        Self {
            _komorebi_thead,
        }
    }
}