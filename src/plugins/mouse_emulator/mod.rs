use bevy::app::{App, Plugin};
use bevy::prelude::{EventReader, EventWriter, Resource, Startup, Update};
use serde::{Deserialize, Serialize};

use crate::plugins::global_input::events::{InputEvent, Key, KeyEvent};
use crate::plugins::input_sequence::events::{Sequence, SubscribeToSequence, Subscription};
use crate::plugins::mouse_emulator::events::MoveMouseUp;
use crate::systems;
use crate::systems::config_loader::Config;

mod events;

const MOUSE_EMULATOR_PLUGIN_NAME: &str = "mouse_emulator";

pub struct MouseEmulatorPlugin;

impl Plugin for MouseEmulatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::config_loader::load_resource::<MouseEmulatorConfig>);

        app.add_event::<MoveMouseUp>();
        app.add_systems(Startup, |mut event_writer: EventWriter<SubscribeToSequence>| {
            let subscription = Subscription {
                subscriber:MOUSE_EMULATOR_PLUGIN_NAME.to_string(),
                sequence:  Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::UpArrow))]),
            };
            let event = SubscribeToSequence(subscription);
            event_writer.send(event);
        });
        app.add_systems(Update, systems::sequence_to_event::sequence_to_event::<MoveMouseUp>(Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::UpArrow))])));
        app.add_systems(Update, on_move_mouse_up);
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

fn on_move_mouse_up(mut events: EventReader<MoveMouseUp>) {
    for event in events.read() {
        println!("{event:?}");
    }
}
