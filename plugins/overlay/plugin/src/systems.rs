use bevy::prelude::{Camera2dBundle, Commands, Entity, NonSend, Query, Window};
use bevy::winit::WinitWindows;

use crate::OVERLAY_PLUGIN_NAME;

// set windowed fullscreen manually
// because of bevy error in case of enabled transparent & borderless & fullscreen properties
pub fn setup_window_system(winit_windows: NonSend<WinitWindows>, mut windows: Query<(Entity, &mut Window)>) {
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

pub fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
