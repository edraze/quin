use bevy::app::{App, Plugin, PluginGroup, Startup};
use bevy::DefaultPlugins;
use bevy::prelude::{Camera2dBundle, ClearColor, Color, Commands, default, Window, WindowPlugin};
use bevy::window::{Cursor, WindowLevel};

const OVERLAY_PLUGIN_NAME: &str = "overlay";

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin { // todo init default plugins in general plugin
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
            .insert_resource(ClearColor(Color::NONE))
            .add_systems(Startup, setup_frame);
    }

    fn name(&self) -> &str {
        OVERLAY_PLUGIN_NAME
    }
}

fn setup_frame(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}