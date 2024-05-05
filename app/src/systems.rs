use std::env;

use auto_launch::AutoLaunchBuilder;
use bevy::prelude::Res;

use crate::config::GeneralConfig;

// todo notification plugin(notify about errors)
pub fn auto_startup_system(config: Res<GeneralConfig>) {
    if let Ok(exe) = env::current_exe() {
        if let Some(exe) = exe.to_str() {
            let auto = AutoLaunchBuilder::new()
                .set_app_name(env!("CARGO_PKG_NAME"))
                .set_app_path(exe)
                .set_use_launch_agent(true)
                .build()
                .unwrap();
            
            let enabled = auto.is_enabled().unwrap_or_default();
            if config.auto_startup && !enabled {
                auto.enable().expect("Error during enabling auto start"); 
            } else if !config.auto_startup && enabled {
                auto.disable().expect("Error during disabling auto start");
            }
            return;
        }
    }
    println!("Can't get executable path");
}