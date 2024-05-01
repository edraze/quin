use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, Window, With};

use global_input_api::filter::{FilterInput, InputFilterEvent};
use input_sequence_api::ToEvent;
use keyboard_to_mouse_plugin::events::{ActivateKeyboardToMouse, DeactivateKeyboardToMouse};
use mouse_output_api::MoveMouseToPosition;
use overlay_plugin::OVERLAY_PLUGIN_NAME;
use toggle::Active;

use crate::components::NavigationPoint;
use crate::events::{ActivateNavigationGrid, DeactivateNavigationGrid, NavigateToLabel};

pub fn init_labels_system(mut commands: Commands, labels: Query<Entity, With<ToEvent<NavigateToLabel>>>,
                          windows: Query<&Window>) {
    let overlay_window = windows.iter()
        .find(|window| window.name == Some(OVERLAY_PLUGIN_NAME.to_string()));
    if let Some(overlay) = overlay_window {
        let overlay_width = overlay.resolution.width() as f64;
        let overlay_height = overlay.resolution.height() as f64;

        let navigation_points = build_navigation_for_rect(overlay_width, overlay_height, 100., 100.); // todo make padding dynamic
        let mut labels = labels.iter();
        for point in navigation_points {
            if let Some(label) = labels.next() {
                commands.entity(label).insert(point);
            }
        }
    }
}

fn build_navigation_for_rect(width: f64, height: f64, x_padding: f64, y_padding: f64) -> Vec<NavigationPoint> {
    let mut result = Vec::new();
    let x_points_count = width / x_padding;
    let y_points_count = height / y_padding;

    for y_index in 0..y_points_count as i32 {
        for x_index in 0..x_points_count as i32 {
            let x = x_index as f64 * x_padding;
            let y = y_index as f64 * y_padding;

            let point = NavigationPoint::new(x, y);
            result.push(point);
        }
    }
    result
}

// todo rename all systems *_system
pub fn on_activate_navigation_grid(mut events: EventReader<ActivateNavigationGrid>, mut writer: EventWriter<InputFilterEvent>,
                                   mut activate_keyboard_to_mouse_writer: EventWriter<ActivateKeyboardToMouse>) {
    if events.read().count() > 0 {
        writer.send(InputFilterEvent::Block(FilterInput::FullKeyboardPress));
        activate_keyboard_to_mouse_writer.send(ActivateKeyboardToMouse); // todo fix keyboard_to_mouse plugin can be disabled by own activation/deactivation sequences
    }
    // todo change grid visibility
}

pub fn on_deactivate_navigation_grid(mut events: EventReader<DeactivateNavigationGrid>, mut writer: EventWriter<InputFilterEvent>,
                                     mut deactivate_keyboard_to_mouse_writer: EventWriter<DeactivateKeyboardToMouse>) {
    if events.read().count() > 0 {
        writer.send(InputFilterEvent::Unblock(FilterInput::FullKeyboardPress));
        deactivate_keyboard_to_mouse_writer.send(DeactivateKeyboardToMouse);
    }
    // todo change grid visibility
}

pub fn on_navigation_system(mut events: EventReader<Active<NavigateToLabel>>,
                            labels: Query<(&ToEvent<NavigateToLabel>, &NavigationPoint)>,
                            mut writer: EventWriter<MoveMouseToPosition>) {
    for Active(event) in events.read() {
        println!("active navigation event");
        let label = labels.iter()
            .find(|(label_event, _)| label_event.event.eq(event));
        if let Some((_, navigation_point)) = label {
            writer.send(MoveMouseToPosition::new(navigation_point.x, navigation_point.y));
        }
    }
}
