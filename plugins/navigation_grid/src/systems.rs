use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, Window, With};

use global_input_api::filter::{FilterInput, InputFilterEvent};
use input_sequence_api::ToEvent;
use keyboard_to_mouse_plugin::events::{ActivateKeyboardToMouse, DeactivateKeyboardToMouse};
use mouse_output_api::MoveMouseToPosition;
use overlay_plugin::OVERLAY_PLUGIN_NAME;
use toggle::Active;
use ui_elements::UiLabel;

use crate::components::NavigationPoint;
use crate::events::{ActivateNavigationGrid, DeactivateNavigationGrid, NavigateToLabel};

pub fn init_labels_system(mut commands: Commands, labels: Query<(Entity, &ToEvent<NavigateToLabel>)>,
                          windows: Query<&Window>) {
    let overlay_window = windows.iter()
        .find(|window| window.name == Some(OVERLAY_PLUGIN_NAME.to_string()));
    if let Some(overlay) = overlay_window {
        let overlay_width = overlay.resolution.width();
        let overlay_height = overlay.resolution.height();

        let ratio = f32::sqrt(labels.iter().count() as f32);
        let x_padding = overlay_width / ratio;
        let y_padding = overlay_height / ratio;

        let mut navigation_points = build_navigation_for_rect(overlay_width, overlay_height, x_padding, y_padding);

        let x_offset = x_padding / 2.;
        let y_offset = y_padding / 2.;
        navigation_points.iter_mut()
            .for_each(|point| point.offset(x_offset, y_offset));

        let mut labels = labels.iter();
        for point in navigation_points {
            if let Some((entity, navigation_event)) = labels.next() {
                let text = &navigation_event.event.label;
                let ui_label = UiLabel::new(false, point.x, point.y, text);

                commands.entity(entity)
                    .insert(point)
                    .insert(ui_label);
            } else {
                println!("WARN navigation point can't be initiated, not enough labels")
            }
        }
    }
}

fn build_navigation_for_rect(width: f32, height: f32, x_padding: f32, y_padding: f32) -> Vec<NavigationPoint> {
    let mut result = Vec::new();
    let x_points_count = width / x_padding;
    let y_points_count = height / y_padding;

    for y_index in 0..y_points_count as i32 {
        for x_index in 0..x_points_count as i32 {
            let x = x_index as f32 * x_padding;
            let y = y_index as f32 * y_padding;

            let point = NavigationPoint::new(x, y);
            result.push(point);
        }
    }
    result
}

// todo rename all systems *_system
pub fn on_activate_navigation_grid(mut events: EventReader<ActivateNavigationGrid>,
                                   mut writer: EventWriter<InputFilterEvent>,
                                   mut activate_keyboard_to_mouse_writer: EventWriter<ActivateKeyboardToMouse>,
                                   mut query: Query<&mut UiLabel, With<ToEvent<NavigateToLabel>>>) {
    if events.read().count() > 0 {
        writer.send(InputFilterEvent::Block(FilterInput::FullKeyboardPress));
        query.iter_mut().for_each(|mut label| label.visible = true);
        activate_keyboard_to_mouse_writer.send(ActivateKeyboardToMouse); // todo fix keyboard_to_mouse plugin can be disabled by own activation/deactivation sequences
    }
}

pub fn on_deactivate_navigation_grid(mut events: EventReader<DeactivateNavigationGrid>,
                                     mut writer: EventWriter<InputFilterEvent>,
                                     mut deactivate_keyboard_to_mouse_writer: EventWriter<DeactivateKeyboardToMouse>,
                                     mut query: Query<&mut UiLabel, With<ToEvent<NavigateToLabel>>>) {
    if events.read().count() > 0 {
        writer.send(InputFilterEvent::Unblock(FilterInput::FullKeyboardPress));
        query.iter_mut().for_each(|mut label| label.visible = false);
        deactivate_keyboard_to_mouse_writer.send(DeactivateKeyboardToMouse);
    }
}

pub fn on_navigation_system(mut events: EventReader<Active<NavigateToLabel>>,
                            labels: Query<(&ToEvent<NavigateToLabel>, &NavigationPoint)>,
                            mut writer: EventWriter<MoveMouseToPosition>) {
    for Active(event) in events.read() {
        println!("active navigation event");
        let label = labels.iter()
            .find(|(label_event, _)| label_event.event.eq(event));
        if let Some((_, navigation_point)) = label {
            writer.send(MoveMouseToPosition::new(navigation_point.x as f64, navigation_point.y as f64));
        }
    }
}
