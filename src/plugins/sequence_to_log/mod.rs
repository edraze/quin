// middleware/handler

use bevy::app::{App, Plugin, Update};
use bevy::prelude::{EventReader, EventWriter, Startup};

use crate::plugins::global_input::events::{InputEvent, Key, KeyEvent};
use crate::plugins::input_sequence::events::{Sequence, SubscribeToSequence, Subscription};

const SEQUENCE_TO_LOG_PLUGIN_NAME: &str = "sequence_to_log";

pub struct SequenceToLogPlugin;

impl Plugin for SequenceToLogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, subscribe_to_sequence);
        app.add_systems(Update, input_to_log);
    }

    fn name(&self) -> &str {
        SEQUENCE_TO_LOG_PLUGIN_NAME
    }
}

fn subscribe_to_sequence(mut event_writer: EventWriter<SubscribeToSequence>) {
    let sequence = Sequence::new(vec![
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyJ)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyJ)),
        InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyK)),
        InputEvent::Keyboard(KeyEvent::Released(Key::KeyK)),
    ]);
    let subscription = Subscription {
        subscriber: SEQUENCE_TO_LOG_PLUGIN_NAME.to_string(),
        sequence,
    };
    let event = SubscribeToSequence(subscription);
    event_writer.send(event);


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
    let subscription = Subscription {
        subscriber: SEQUENCE_TO_LOG_PLUGIN_NAME.to_string(),
        sequence,
    };
    let event = SubscribeToSequence(subscription);
    event_writer.send(event);
}

fn input_to_log(mut events: EventReader<Sequence>) {
    for event in events.read() {
        println!("{event:?}");
    }
}
