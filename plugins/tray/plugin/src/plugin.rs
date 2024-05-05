use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::embedded_asset;

use tray_api::{CreateTrayItem, TrayClick, TrayItemClick};

use crate::systems::{create_tray_system, emmit_tray_event_system, emmit_tray_item_event_system, exit_system, init_tray_system};

const TRAY_PLUGIN_NAME: &str = "tray";


#[derive(Default)]
pub struct TrayPlugin;

impl Plugin for TrayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<TrayClick>()
            .add_event::<TrayItemClick>()
            .add_event::<CreateTrayItem>()

            .add_systems(Startup, init_tray_system)
            .add_systems(Update, (
                create_tray_system,
                emmit_tray_event_system,
                emmit_tray_item_event_system,
                exit_system,
            ));
    }

    fn name(&self) -> &str {
        TRAY_PLUGIN_NAME
    }
}