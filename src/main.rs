use bevy::prelude::*;
use bevy::window::{Cursor, WindowLevel};
use bevy_egui::EguiPlugin;
use crate::plugins::configurator::ConfiguratorPlugin;

use crate::plugins::exit::ExitPlugin;
use crate::plugins::gui::{gui_handler, GuiEvent};
use crate::plugins::input_to_gui::input_to_gui_event;
use crate::plugins::tray::TrayPlugin;

mod plugins;

fn main() {
    let _tray_icon = plugins::tray::build_tray(&["Exit".to_string()]);

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                cursor: Cursor {
                    hit_test: false,
                    ..Cursor::default()
                },
                transparent: true,
                decorations: false,
                // resizable: false,
                // focused: false,
                // mode: WindowMode::BorderlessFullscreen,
                window_level: WindowLevel::AlwaysOnTop,
                #[cfg(target_os = "macos")]
                composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                ..default()
            }),
            ..default()
        }).set(bevy::log::LogPlugin {
            // level: bevy::log::Level::TRACE,
            // filter: "wgpu=warn,bevy_ecs=info".to_string(),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins(ConfiguratorPlugin)
        .add_plugins(TrayPlugin)
        .add_plugins(ExitPlugin)
        .insert_resource(ClearColor(Color::NONE))
        .add_systems(Startup, setup)

        //  todo remove, only for dev
        .add_event::<GuiEvent>()

        .add_systems(Update, (
            gui_handler,
            input_to_gui_event,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
