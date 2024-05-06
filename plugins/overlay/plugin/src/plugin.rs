use bevy::app::{App, Plugin, PluginGroup, Startup};
use bevy::DefaultPlugins;
use bevy::prelude::{ClearColor, Color, default, IVec2, Window, WindowPlugin, WindowPosition};
use bevy::window::{Cursor, WindowLevel, WindowResolution};
use crate::systems::{setup_camera_system, setup_window_system};

pub const OVERLAY_PLUGIN_NAME: &str = "overlay";

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin { // todo init default plugins in general plugin
            primary_window: Some(Window {
                name: Some(OVERLAY_PLUGIN_NAME.to_string()),
                cursor: Cursor {
                    hit_test: false,
                    ..Cursor::default()
                },
                transparent: true,
                decorations: false,
                // resizable: false,
                focused: false,
                // mode: WindowMode::BorderlessFullscreen,
                resolution: WindowResolution::new(2560., 1440.),
                position: WindowPosition::At(IVec2::ZERO),
                window_level: WindowLevel::AlwaysOnTop,
                // todo uncomment with bevy 0.14 (https://github.com/bevyengine/bevy/pull/12450)
                // skip_taskbar: true,
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
            .add_systems(Startup, (
                setup_window_system,
                setup_camera_system,
            ));
    }

    fn name(&self) -> &str {
        OVERLAY_PLUGIN_NAME
    }
}
