// middleware/handler

use std::fmt::Debug;

use bevy::app::{App, Plugin};
use bevy::prelude::{Event, EventReader, Update};
use global_input_api::input::InputEvent;
use global_input_api::input_model::keyboard::{Key, KeyEvent};
use input_sequence_api::Sequence;
use input_sequence_plugin::listen_sequences;

const SEQUENCE_TO_LOG_PLUGIN_NAME: &str = "sequence_to_log";

pub struct SequenceToLogPlugin;

impl Plugin for SequenceToLogPlugin {
    fn build(&self, app: &mut App) {
        let sequence = Sequence::new(vec![
            InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyC)),
            InputEvent::Keyboard(KeyEvent::Released(Key::KeyC)),
            InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyO)),
            InputEvent::Keyboard(KeyEvent::Released(Key::KeyO)),
            InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyD)),
            InputEvent::Keyboard(KeyEvent::Released(Key::KeyD)),
            InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyE)),
            InputEvent::Keyboard(KeyEvent::Released(Key::KeyE)),
        ]);
        listen_sequences(app, (sequence, SequenceToLog));
        app.add_systems(Update, input_to_log_system);
    }

    fn name(&self) -> &str {
        SEQUENCE_TO_LOG_PLUGIN_NAME
    }
}

fn input_to_log_system(mut events: EventReader<SequenceToLog>) {
    for event in events.read() {
        println!("{event:?}");
    }
}

#[derive(Event, Clone, Debug)]
struct SequenceToLog;
