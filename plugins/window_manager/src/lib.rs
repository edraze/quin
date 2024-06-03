#![cfg(target_os = "windows")]

pub use plugin::*;

mod systems;
mod state;
mod config;
mod plugin;
mod events;

