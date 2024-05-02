use bevy::app::{App, Plugin, Startup};
use bevy::prelude::Update;
use itertools::Itertools;

use global_input_api::input::InputEvent;
use global_input_api::input_model::keyboard::{Key, KeyEvent};
use input_sequence_api::Sequence;
use keyboard_layout::Layout;
use keyboard_to_mouse_plugin::config::KeyboardToMouseConfig;

use crate::config::NavigationGridConfig;
use crate::events::{ActivateNavigationGrid, DeactivateNavigationGrid, NavigateToLabel};
use crate::systems::{init_labels_system, on_activate_navigation_grid, on_deactivate_navigation_grid, on_navigation_system};

pub(crate) const NAVIGATION_GRID_PLUGIN_NAME: &str = "navigation_grid";

pub struct NavigationGridPlugin;

impl Plugin for NavigationGridPlugin {
    fn build(&self, app: &mut App) {
        let config = config_loader::load::<NavigationGridConfig>();
        app.insert_resource(config.clone());

        let activation_binding = (config.key_bindings.activate.clone(), ActivateNavigationGrid).into();
        let deactivation_binding = (config.key_bindings.deactivate.clone(), DeactivateNavigationGrid).into();

        let allowed_keys = config.allowed_label_key.chars().flat_map(Key::try_from);
        let forbidden_keys = get_keyboard_to_mouse_keys(app);

        let label_keys: Vec<Vec<Key>> = allowed_keys
            .filter(|key| !forbidden_keys.contains(key))
            .permutations(2)
            .collect();

        let mut layout = Layout::new(app, activation_binding, deactivation_binding);
        for keys in label_keys {
            let label_sequence = keys_to_sequences(keys.clone()); // todo remove this clone
            let label_text = keys_to_string(keys);
            let event = NavigateToLabel::new(&label_text);
            layout = layout.bind((label_sequence, event));
        }

        app.add_systems(Startup, init_labels_system);
        app.add_systems(Update, (
            on_activate_navigation_grid,
            on_deactivate_navigation_grid,
            on_navigation_system
        ));
    }

    fn name(&self) -> &str {
        NAVIGATION_GRID_PLUGIN_NAME
    }
}

fn get_keyboard_to_mouse_keys(app: &App) -> Vec<Key> {
    let keyboard_to_mouse_config = app.world.get_resource::<KeyboardToMouseConfig>();
    let keyboard_to_mouse_sequences = if let Some(config) = keyboard_to_mouse_config {
        let bindings = config.key_bindings.clone();
        // todo may be it will better to create method on binding config struct (trait?)
        vec![
            bindings.activate,
            bindings.deactivate,
            bindings.mouse_move_up,
            bindings.mouse_move_down,
            bindings.mouse_move_left,
            bindings.mouse_move_right,
            bindings.mouse_scroll_up,
            bindings.mouse_scroll_down,
            bindings.mouse_scroll_left,
            bindings.mouse_scroll_right,
            bindings.mouse_left_button_click,
            bindings.mouse_middle_button_click,
            bindings.mouse_right_button_click,
            bindings.mouse_drag_and_drop_activate,
            bindings.mouse_drag_and_drop_deactivate,
        ]
    } else {
        Vec::new()
    };

    keyboard_to_mouse_sequences.into_iter()
        .flatten()
        .flat_map(sequence_to_keys)
        .unique()
        .collect()
}

fn sequence_to_keys(sequence: Sequence) -> Vec<Key> {
    let mut keys = Vec::new();
    for input_event in sequence.input_events.into_iter() {
        if let InputEvent::Keyboard(key_event) = input_event {
            let key = match key_event {
                KeyEvent::Pressed(key) => key,
                KeyEvent::Released(key) => key,
            };
            keys.push(key);
        }
    }
    keys
}

fn keys_to_sequences(keys: Vec<Key>) -> Sequence {
    let mut input_events = Vec::new();
    for key in keys.into_iter() {
        input_events.push(InputEvent::Keyboard(KeyEvent::Pressed(key)));
        input_events.push(InputEvent::Keyboard(KeyEvent::Released(key)));
    }
    Sequence::new(input_events)
}

fn keys_to_string(keys: Vec<Key>) -> String {
    keys.into_iter()
        .flat_map(|key| char::try_from(key))
        .collect()
}