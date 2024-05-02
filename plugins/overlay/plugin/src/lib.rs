use bevy::app::{App, Plugin, PluginGroup, Startup};
use bevy::DefaultPlugins;
use bevy::prelude::{Camera2dBundle, ClearColor, Color, Commands, default, Entity, IVec2, NonSend, Query, Window, WindowPlugin, WindowPosition};
use bevy::window::{Cursor, WindowLevel, WindowResolution};
use bevy::winit::WinitWindows;

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
                // focused: false,
                // mode: WindowMode::BorderlessFullscreen,
                resolution: WindowResolution::new(2560., 1440.),
                position: WindowPosition::At(IVec2::ZERO),
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
            .add_systems(Startup, (
                setup_window,
                setup_camera,
            ));
    }

    fn name(&self) -> &str {
        OVERLAY_PLUGIN_NAME
    }
}

// set windowed fullscreen manually
// because of bevy crash in case of transparent & borderless & fullscreen window
fn setup_window(winit_windows: NonSend<WinitWindows>, mut windows: Query<(Entity, &mut Window)>) {
    let overlay = windows.iter_mut()
        .find(|(_, window)| window.name == Some(OVERLAY_PLUGIN_NAME.to_string()));
    if let Some((entity, mut window)) = overlay {
        let monitor = winit_windows.get_window(entity)
            .and_then(|winit_window| winit_window.current_monitor())
            .map(|monitor| {
                let size = monitor.size();
                window.resolution.set(size.width as f32, size.height as f32);
                
            });
        if monitor.is_some(){
            println!("Overlay resolution changed")
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
