use bevy::app::{App, Plugin};
use bevy::prelude::{EventWriter, Resource, Startup, Update};
use serde::{Deserialize, Serialize};
use config_loader_plugin::Config;
use global_input_api::{InputEvent, Key, KeyEvent};
use input_sequence_api::{Sequence, SubscribeToSequence, Subscription};
use mouse_output_api::MoveMouseRelatively;
use sequence_to_event::sequence_to_event;

const MOUSE_EMULATOR_PLUGIN_NAME: &str = "mouse_emulator";

pub struct MouseEmulatorPlugin;

impl Plugin for MouseEmulatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, config_loader_plugin::load_resource::<MouseEmulatorConfig>);

        app.add_systems(Startup, |mut event_writer: EventWriter<SubscribeToSequence>| {
            let subscription = Subscription {
                subscriber:MOUSE_EMULATOR_PLUGIN_NAME.to_string(),
                sequence:  Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::UpArrow))]),
            };
            let event = SubscribeToSequence(subscription);
            event_writer.send(event);
        });
        app.add_systems(Update, sequence_to_event::<MoveMouseRelatively>(Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::UpArrow))])));
    }

    fn name(&self) -> &str {
        MOUSE_EMULATOR_PLUGIN_NAME
    }
}

#[derive(Resource, Serialize, Deserialize, Default, Debug)]
pub struct MouseEmulatorConfig {
    key_binding: MouseEmulatorKeyBindings,
    scroll_speed: i64, // todo i8
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MouseEmulatorKeyBindings {
    // activate: , // Vec<Vec<InputEvent>>
    // deactivate: ,
    // mouse_move_up: ,
    // mouse_move_down: ,
    // mouse_move_left: ,
    // mouse_move_right: ,
    // mouse_left_button_click: ,
    // mouse_right_button_click:,
    // mouse_middle_button_click:,
    // mouse_scroll_up:,
    // mouse_scroll_down: ,
    // mouse_drag_and_drop_activate: ,
    // mouse_drag_and_drop_deactivate: ,
}

impl Config for MouseEmulatorConfig {
    fn name() -> String {
        MOUSE_EMULATOR_PLUGIN_NAME.to_string()
    }
}

