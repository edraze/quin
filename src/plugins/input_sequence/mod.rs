use bevy::app::{App, Plugin};
use bevy::prelude::{EventReader, EventWriter, ResMut, Resource, Update};

use crate::plugins::global_input::events::InputEvent;
use crate::plugins::input_sequence::events::{Sequence, SubscribeToSequence, Subscription, UnsubscribeToSequence};

pub mod events;

const INPUT_SEQUENCE_PLUGIN_NAME: &str = "input-sequence";

pub struct InputSequencePlugin;

impl Plugin for InputSequencePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputSequenceState>();
        app.add_event::<SubscribeToSequence>();
        app.add_event::<UnsubscribeToSequence>();
        app.add_event::<Sequence>();
        app.add_systems(Update, subscribe_to_sequence);
        app.add_systems(Update, unsubscribe_to_sequence);
        app.add_systems(Update, input_to_sequence_event);
    }

    fn name(&self) -> &str {
        INPUT_SEQUENCE_PLUGIN_NAME
    }
}

fn subscribe_to_sequence(mut state: ResMut<InputSequenceState>, mut events: EventReader<SubscribeToSequence>) {
    for event in events.read().cloned() {
        println!("subscription: {:?}", event.0);
        state.subscribe(event.0)
    }
}

fn unsubscribe_to_sequence(mut state: ResMut<InputSequenceState>, mut events: EventReader<UnsubscribeToSequence>) {
    for event in events.read().cloned() {
        state.unsubscribe(event.0)
    }
}

fn input_to_sequence_event(mut state: ResMut<InputSequenceState>, mut input_events: EventReader<InputEvent>, mut sequence_events: EventWriter<Sequence>) {
    for input_event in input_events.read().cloned() {
        println!("input: {:?}", input_event);
        let sequences = state.input_to_sequences(input_event);
        sequence_events.send_batch(sequences);
    }
}

#[derive(Resource, Default)]
struct InputSequenceState {
    records: Vec<Record>,
    input_buffer: InputBuffer,
}

impl InputSequenceState {
    pub fn subscribe(&mut self, subscription: Subscription) {
        if let Some(record) = self.get_record_mut(&subscription.sequence) {
            record.subscribers.push(subscription.subscriber.clone());
        } else {
            let buffer_len = subscription.sequence.len();
            self.records.push(Record::from_subscription(subscription));
            self.input_buffer = InputBuffer::from_input_buffer(&mut self.input_buffer, buffer_len)
        }
    }

    pub fn unsubscribe(&mut self, subscription: Subscription) {
        let mut need_remove = false;
        if let Some(record) = self.get_record_mut(&subscription.sequence) {
            record.subscribers.retain(|subscriber| !subscription.subscriber.eq(subscriber));
            need_remove = record.subscribers.is_empty();
        }
        if need_remove {
            self.remove(&subscription.sequence)
        }
    }

    pub fn input_to_sequences(&mut self, input: InputEvent) -> Vec<Sequence> {
        self.input_buffer.push(input);
        self.records.iter()
            .map(|record| record.sequence.clone()) // todo clone only required sequences not all
            .filter(|sequence| self.input_buffer.ends_with(sequence))
            .collect()
    }

    fn get_record_mut(&mut self, sequence: &Sequence) -> Option<&mut Record> {
        self.records
            .iter_mut()
            .find(|record| record.sequence.eq(sequence))
    }

    fn remove(&mut self, sequence: &Sequence) {
        self.records
            .retain(|record| !record.sequence.eq(sequence))
    }
}

// todo rename
#[derive(Debug)]
struct Record {
    subscribers: Vec<String>,
    sequence: Sequence,
}

impl Record {
    pub fn new(subscriber: String, sequence: Sequence) -> Self {
        Self {
            subscribers: vec![subscriber],
            sequence,
        }
    }

    pub fn from_subscription(subscription: Subscription) -> Self {
        Self::new(subscription.subscriber, subscription.sequence)
    }
}

#[derive(Default)]
struct InputBuffer {
    buffer: Vec<InputEvent>,
}

impl InputBuffer {
    pub fn from_input_buffer(existing_input_buffer: &mut InputBuffer, capacity: usize) -> Self {
        let mut input_buffer = Vec::with_capacity(capacity);
        input_buffer.append(&mut existing_input_buffer.buffer);
        Self {
            buffer: input_buffer
        }
    }

    pub fn push(&mut self, input: InputEvent) {
        if self.buffer.capacity() != 0 {
            if self.buffer.len() == self.buffer.capacity() {
                self.buffer.remove(0);
            }
            self.buffer.push(input);
        }
    }

    pub fn ends_with(&self, sequence: &Sequence) -> bool {
        self.buffer.ends_with(&sequence.sequence)
    }
}