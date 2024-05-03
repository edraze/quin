use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, Window, With};

use global_input_api::filter::{FilterInput, InputFilterEvent};
use input_sequence_api::{ResetSequenceBuffer, ToEvent};
use keyboard_to_mouse_plugin::events::{ActivateKeyboardToMouse, DeactivateKeyboardToMouse};
use mouse_output_api::MoveMouseToPosition;
use overlay_plugin::OVERLAY_PLUGIN_NAME;
use toggle::Active;
use ui_elements::UiLabel;

use crate::components::NavigationPoint;
use crate::events::{ActivateMainGrid, ActivateNavigationGrid, ActivateSubGrid, DeactivateMainGrid, DeactivateNavigationGrid, DeactivateSubGrid, NavigateToLabel, NavigateToSubLabel, UpdateSubGridPosition};

pub fn init_labels_system(windows: Query<&Window>,
                          labels: Query<(Entity, &ToEvent<NavigateToLabel>)>,
                          sub_labels: Query<(Entity, &ToEvent<NavigateToSubLabel>)>,
                          mut commands: Commands) {
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

        let sub_grid_width = x_padding * 2.;
        let sub_grid_height = y_padding * 2.;
        let sub_grid_density = 20.;
        let navigation_sub_points = build_navigation_for_rect(sub_grid_width, sub_grid_height, sub_grid_density, sub_grid_density);

        let mut sub_labels = sub_labels.iter();
        for point in navigation_sub_points {
            if let Some((entity, navigation_event)) = sub_labels.next() {
                let text = &navigation_event.event.label;
                let ui_label = UiLabel::new(false, point.x, point.y, text);

                commands.entity(entity)
                    .insert(point)
                    .insert(ui_label);
            } else {
                println!("WARN navigation sub point can't be initiated, not enough labels")
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
pub fn activate_plugin_system(mut activate_plugin_reader: EventReader<ActivateNavigationGrid>,
                              mut input_filter_writer: EventWriter<InputFilterEvent>,
                              mut activate_main_grid_writer: EventWriter<ActivateMainGrid>,
                              mut activate_keyboard_to_mouse_writer: EventWriter<ActivateKeyboardToMouse>,
) {
    if activate_plugin_reader.read().count() > 0 {
        input_filter_writer.send(InputFilterEvent::Block(FilterInput::FullKeyboardPress));
        activate_main_grid_writer.send(ActivateMainGrid);
        activate_keyboard_to_mouse_writer.send(ActivateKeyboardToMouse); // todo fix keyboard_to_mouse plugin can be disabled by own activation/deactivation sequences
    }
}

pub fn deactivate_plugin_system(mut deactivate_plugin_reader: EventReader<DeactivateNavigationGrid>,
                                mut input_filter_writer: EventWriter<InputFilterEvent>,
                                mut deactivate_main_grid_writer: EventWriter<DeactivateMainGrid>,
                                mut deactivate_sub_grid_writer: EventWriter<DeactivateSubGrid>,
                                mut deactivate_keyboard_to_mouse_writer: EventWriter<DeactivateKeyboardToMouse>,
) {
    if deactivate_plugin_reader.read().count() > 0 {
        deactivate_keyboard_to_mouse_writer.send(DeactivateKeyboardToMouse);
        deactivate_sub_grid_writer.send(DeactivateSubGrid);
        deactivate_main_grid_writer.send(DeactivateMainGrid);
        input_filter_writer.send(InputFilterEvent::Unblock(FilterInput::FullKeyboardPress));
    }
}

pub fn activate_main_grid_system(mut activate_main_grid_reader: EventReader<ActivateMainGrid>,
                                 mut query: Query<&mut UiLabel, With<ToEvent<NavigateToLabel>>>) {
    if activate_main_grid_reader.read().count() > 0 {
        query.iter_mut().for_each(|mut label| label.visible = true);
    }
}

pub fn deactivate_main_grid_system(mut deactivate_main_grid_reader: EventReader<DeactivateMainGrid>,
                                   mut query: Query<&mut UiLabel, With<ToEvent<NavigateToLabel>>>) {
    if deactivate_main_grid_reader.read().count() > 0 {
        query.iter_mut().for_each(|mut label| label.visible = false);
    }
}


pub fn activate_sub_grid_system(mut activate_sub_grid_reader: EventReader<ActivateSubGrid>,
                                mut query: Query<&mut UiLabel, With<ToEvent<NavigateToSubLabel>>>) {
    if activate_sub_grid_reader.read().count() > 0 {
        query.iter_mut().for_each(|mut label| label.visible = true);
    }
}

pub fn navigate_to_label_system(mut navigate_event_reader: EventReader<Active<NavigateToLabel>>,
                                labels: Query<(&ToEvent<NavigateToLabel>, &NavigationPoint)>,
                                mut update_sub_grid_writer: EventWriter<UpdateSubGridPosition>,
                                mut move_mouse_writer: EventWriter<MoveMouseToPosition>,
                                mut deactivate_main_grid_writer: EventWriter<DeactivateMainGrid>,
                                mut reset_sequence_buffer_writer: EventWriter<ResetSequenceBuffer>,
                                mut activate_sub_grid_writer: EventWriter<ActivateSubGrid>,
) {
    for Active(event) in navigate_event_reader.read() {
        println!("navigate to main point");
        let label = labels.iter()
            .find(|(label_event, _)| label_event.event.eq(event));
        if let Some((_, navigation_point)) = label {
            let update_sub_grid_event = UpdateSubGridPosition::new(navigation_point.x, navigation_point.y);
            update_sub_grid_writer.send(update_sub_grid_event);
            move_mouse_writer.send(MoveMouseToPosition::new(navigation_point.x as f64, navigation_point.y as f64));
            deactivate_main_grid_writer.send(DeactivateMainGrid);
            reset_sequence_buffer_writer.send(ResetSequenceBuffer);
            activate_sub_grid_writer.send(ActivateSubGrid);
        }
    }
}

pub fn deactivate_sub_grid_system(mut deactivate_sub_grid_reader: EventReader<DeactivateSubGrid>,
                                  mut query: Query<&mut UiLabel, With<ToEvent<NavigateToSubLabel>>>) {
    if deactivate_sub_grid_reader.read().count() > 0 {
        query.iter_mut().for_each(|mut label| label.visible = false);
    }
}

pub fn update_sub_grid_position(mut update_sub_grid_reader: EventReader<UpdateSubGridPosition>) {}

pub fn navigate_to_sub_label_system(mut navigate_sub_label_reader: EventReader<Active<NavigateToSubLabel>>,
                                    labels: Query<(&ToEvent<NavigateToSubLabel>, &NavigationPoint)>,
                                    mut move_mouse_writer: EventWriter<MoveMouseToPosition>,
                                    mut deactivate_sub_grid_writer: EventWriter<DeactivateSubGrid>,
                                    mut reset_sequence_buffer_writer: EventWriter<ResetSequenceBuffer>,
                                    mut activate_main_grid_writer: EventWriter<ActivateMainGrid>,
) {
    for Active(event) in navigate_sub_label_reader.read() {
        println!("navigate to sub point");
        let label = labels.iter()
            .find(|(label_event, _)| label_event.event.eq(event));
        if let Some((_, navigation_point)) = label {
            move_mouse_writer.send(MoveMouseToPosition::new(navigation_point.x as f64, navigation_point.y as f64));
            deactivate_sub_grid_writer.send(DeactivateSubGrid);
            reset_sequence_buffer_writer.send(ResetSequenceBuffer);
            activate_main_grid_writer.send(ActivateMainGrid);
        }
    }
}