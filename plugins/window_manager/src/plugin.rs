use bevy::app::{App, Plugin, Update};

use input_sequence_api::SequencesToEvent;
use input_sequence_plugin::listen_sequences;

use crate::config::TilingWindowManagerConfig;
use crate::events::{CloseWindow, FocusWindowDown, FocusWindowLeft, FocusWindowRight, FocusWindowUp, MinimizeWindow, MoveWindowDown, MoveWindowLeft, MoveWindowRight, MoveWindowUp, StackWindowDown, StackWindowLeft, StackWindowRight, StackWindowUp, ToggleFloat, ToggleMaximize, ToggleMonocle, UnstackWindow};
use crate::state::KomorebiState;
use crate::systems::{on_close_window_system, on_focus_down_system, on_focus_left_system, on_focus_right_system, on_focus_up_system, on_minimize_window_system, on_move_down_system, on_move_left_system, on_move_right_system, on_move_up_system, on_stack_down_system, on_stack_left_system, on_stack_right_system, on_stack_up_system, on_toggle_float_system, on_toggle_maximize_system, on_toggle_monocle_system, on_unstack_system};

pub const TILING_WINDOW_MANAGER_PLUGIN_NAME: &str = "tiling_window_manager";

#[derive(Default)]
pub struct TilingWindowManagerPlugin;

impl Plugin for TilingWindowManagerPlugin {
    fn build(&self, app: &mut App) {
        let config = config_loader::load::<TilingWindowManagerConfig>();

        let focus_left_binding: SequencesToEvent<_> = (config.key_bindings.focus_left.clone(), FocusWindowLeft).into();
        listen_sequences(app, focus_left_binding);
        let focus_right_binding: SequencesToEvent<_> = (config.key_bindings.focus_right.clone(), FocusWindowRight).into();
        listen_sequences(app, focus_right_binding);
        let focus_up_binding: SequencesToEvent<_> = (config.key_bindings.focus_up.clone(), FocusWindowUp).into();
        listen_sequences(app, focus_up_binding);
        let focus_down_binding: SequencesToEvent<_> = (config.key_bindings.focus_down.clone(), FocusWindowDown).into();
        listen_sequences(app, focus_down_binding);
        let move_left_binding: SequencesToEvent<_> = (config.key_bindings.move_left.clone(), MoveWindowLeft).into();
        listen_sequences(app, move_left_binding);
        let move_right_binding: SequencesToEvent<_> = (config.key_bindings.move_right.clone(), MoveWindowRight).into();
        listen_sequences(app, move_right_binding);
        let move_up_binding: SequencesToEvent<_> = (config.key_bindings.move_up.clone(), MoveWindowUp).into();
        listen_sequences(app, move_up_binding);
        let move_down_binding: SequencesToEvent<_> = (config.key_bindings.move_down.clone(), MoveWindowDown).into();
        listen_sequences(app, move_down_binding);
        let stack_left_binding: SequencesToEvent<_> = (config.key_bindings.stack_left.clone(), StackWindowLeft).into();
        listen_sequences(app, stack_left_binding);
        let stack_right_binding: SequencesToEvent<_> = (config.key_bindings.stack_right.clone(), StackWindowRight).into();
        listen_sequences(app, stack_right_binding);
        let stack_up_binding: SequencesToEvent<_> = (config.key_bindings.stack_up.clone(), StackWindowUp).into();
        listen_sequences(app, stack_up_binding);
        let stack_down_binding: SequencesToEvent<_> = (config.key_bindings.stack_down.clone(), StackWindowDown).into();
        listen_sequences(app, stack_down_binding);
        let unstack_binding: SequencesToEvent<_> = (config.key_bindings.unstack.clone(), UnstackWindow).into();
        listen_sequences(app, unstack_binding);
        let toggle_maximize_binding: SequencesToEvent<_> = (config.key_bindings.toggle_maximize.clone(), ToggleMaximize).into();
        listen_sequences(app, toggle_maximize_binding);
        let toggle_monocle_binding: SequencesToEvent<_> = (config.key_bindings.toggle_monocle.clone(), ToggleMonocle).into();
        listen_sequences(app, toggle_monocle_binding);
        let toggle_float_binding: SequencesToEvent<_> = (config.key_bindings.toggle_float.clone(), ToggleFloat).into();
        listen_sequences(app, toggle_float_binding);
        let minimize_window_binding: SequencesToEvent<_> = (config.key_bindings.minimize.clone(), MinimizeWindow).into();
        listen_sequences(app, minimize_window_binding);
        let close_window_binding: SequencesToEvent<_> = (config.key_bindings.close.clone(), CloseWindow).into();
        listen_sequences(app, close_window_binding);

        let state = KomorebiState::init(komorebi::KOMOREBI_CONFIG, komorebi::KOMOREBI_APPLICATIONS_CONFIG);
        app
            .insert_resource(state)
            .add_systems(Update, (
                on_focus_left_system,
                on_focus_right_system,
                on_focus_up_system,
                on_focus_down_system,
                on_move_left_system,
                on_move_right_system,
                on_move_up_system,
                on_move_down_system,
                on_stack_left_system,
                on_stack_right_system,
                on_stack_up_system,
                on_stack_down_system,
                on_unstack_system,
                on_toggle_maximize_system,
                on_toggle_monocle_system,
                on_toggle_float_system,
                on_minimize_window_system,
                on_close_window_system
            ));
    }

    fn name(&self) -> &str {
        TILING_WINDOW_MANAGER_PLUGIN_NAME
    }
}

