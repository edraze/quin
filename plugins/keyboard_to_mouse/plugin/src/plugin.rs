use bevy::app::{App, Plugin, Update};
use input_sequence_api::SequencesToEvent;
use input_sequence_plugin::listen_sequences;
use keyboard_layout::Layout;
use crate::config::KeyboardToMouseConfig;
use crate::events::{ActivateKeyboardToMouse, DeactivateKeyboardToMouse, DragAndDropEnd, DragAndDropStart, MouseLeftButtonClick, MouseMiddleButtonClick, MouseRightButtonClick, MoveMouseRelativelyDown, MoveMouseRelativelyLeft, MoveMouseRelativelyRight, MoveMouseRelativelyUp, ScrollDown, ScrollLeft, ScrollRight, ScrollUp};
use crate::systems::{on_activate_keyboard_to_mouse_system, on_deactivate_keyboard_to_mouse_system, on_drag_and_drop_end_system, on_drag_and_drop_start_system, on_mouse_left_button_click_system, on_mouse_middle_button_click_system, on_mouse_right_button_click_system, on_move_mouse_relatively_down_system, on_move_mouse_relatively_left_system, on_move_mouse_relatively_right_system, on_move_mouse_relatively_up_system, on_scroll_down_system, on_scroll_left_system, on_scroll_right_system, on_scroll_up_system};

pub(crate) const KEYBOARD_TO_MOUSE_PLUGIN_NAME: &str = "keyboard_to_mouse";

pub struct KeyboardToMousePlugin;

impl Plugin for KeyboardToMousePlugin {
    fn build(&self, app: &mut App) {
        let config = config_loader::load::<KeyboardToMouseConfig>();
        app.insert_resource(config.clone());

        let activation_binding: SequencesToEvent<_> = (config.key_bindings.activate.clone(), ActivateKeyboardToMouse).into();
        let deactivation_binding: SequencesToEvent<_> = (config.key_bindings.deactivate.clone(), DeactivateKeyboardToMouse).into();
        listen_sequences(app, activation_binding);
        listen_sequences(app, deactivation_binding);
        Layout::<ActivateKeyboardToMouse, DeactivateKeyboardToMouse>::new(app)
            .bind((config.key_bindings.mouse_move_up.clone(), MoveMouseRelativelyUp))
            .bind((config.key_bindings.mouse_move_down.clone(), MoveMouseRelativelyDown))
            .bind((config.key_bindings.mouse_move_left.clone(), MoveMouseRelativelyLeft))
            .bind((config.key_bindings.mouse_move_right.clone(), MoveMouseRelativelyRight))

            .bind((config.key_bindings.mouse_scroll_up.clone(), ScrollUp))
            .bind((config.key_bindings.mouse_scroll_down.clone(), ScrollDown))
            .bind((config.key_bindings.mouse_scroll_left.clone(), ScrollLeft))
            .bind((config.key_bindings.mouse_scroll_right.clone(), ScrollRight))

            .bind((config.key_bindings.mouse_left_button_click.clone(), MouseLeftButtonClick))
            .bind((config.key_bindings.mouse_middle_button_click.clone(), MouseMiddleButtonClick))
            .bind((config.key_bindings.mouse_right_button_click.clone(), MouseRightButtonClick))

            .bind((config.key_bindings.mouse_drag_and_drop_activate.clone(), DragAndDropStart))
            .bind((config.key_bindings.mouse_drag_and_drop_deactivate.clone(), DragAndDropEnd));

        app.add_systems(Update, (
            on_activate_keyboard_to_mouse_system,
            on_deactivate_keyboard_to_mouse_system,
            on_move_mouse_relatively_up_system,
            on_move_mouse_relatively_down_system,
            on_move_mouse_relatively_left_system,
            on_move_mouse_relatively_right_system,
            on_scroll_up_system,
            on_scroll_down_system,
            on_scroll_left_system,
            on_scroll_right_system,
            on_mouse_left_button_click_system,
            on_mouse_middle_button_click_system,
            on_mouse_right_button_click_system,
            on_drag_and_drop_start_system,
            on_drag_and_drop_end_system
        ));
    }

    fn name(&self) -> &str {
        KEYBOARD_TO_MOUSE_PLUGIN_NAME
    }
}
