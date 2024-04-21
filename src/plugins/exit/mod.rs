use bevy::app::{App, Plugin, Update};
use bevy::prelude::EventReader;

use crate::plugins::tray::events::TrayItemClick;

const EXIT_PLUGIN_NAME: &str = "exit";
const EXIT_TRAY_ITEM_ID: &str = "Exit";

pub struct ExitPlugin;

impl Plugin for ExitPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Startup, add_exit_tray_item)
            .add_systems(Update, exit);
    }

    fn name(&self) -> &str {
        EXIT_PLUGIN_NAME
    }
}

// fn add_exit_tray_item(mut create_tray_item: EventWriter<CreateTrayItem>) {
//     let event = CreateTrayItem::new(EXIT_TRAY_ITEM_ID);
//     create_tray_item.send(event);
// }

fn exit(mut events: EventReader<TrayItemClick>) {
    for event in events.read() {
        if event.id == EXIT_TRAY_ITEM_ID {
            std::process::exit(0);
        }
    }
}
