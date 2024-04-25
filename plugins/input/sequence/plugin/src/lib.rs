use bevy::app::{App, Plugin};
use bevy::prelude::{Added, Changed, Component, DetectChanges, Event, EventReader, Events, EventWriter, Or, Query, Res, ResMut, Resource, Update, World};

use global_input_api::input::InputEvent;
use input_sequence_api::Sequence;

const INPUT_SEQUENCE_PLUGIN_NAME: &str = "input_sequence";

pub struct InputSequencePlugin;

impl Plugin for InputSequencePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SequenceBuffer>();
        app.add_systems(Update, on_input_event);
        app.add_systems(Update, update_buffer);
    }

    fn name(&self) -> &str {
        INPUT_SEQUENCE_PLUGIN_NAME
    }
}

pub fn listen_sequences<E: Event + Clone>(app: &mut App, sequences: Vec<Sequence>, event: ToEvent<E>) {
    if !app.world.contains_resource::<Events<E>>() {
        app.add_event::<E>();
        app.add_systems(Update, check_sequence::<E>);
    }
    for sequence in sequences {
        subscribe(&mut app.world, sequence, event.clone());
    }
}

pub fn listen_sequence<E: Event + Clone>(app: &mut App, sequence: Sequence, event: ToEvent<E>) {
    if !app.world.contains_resource::<Events<E>>() {
        app.add_event::<E>();
        app.add_systems(Update, check_sequence::<E>);
    }
    subscribe(&mut app.world, sequence, event);
}

pub fn subscribe<E: Event + Clone>(world: &mut World, sequence: Sequence, event: ToEvent<E>) {
    world.spawn((sequence, event));
}

// todo implements unsubscribe (remove entity with Sequence and ToEvent components)

fn on_input_event(mut buffer: ResMut<SequenceBuffer>, mut events: EventReader<InputEvent>) {
    for event in events.read().cloned() {
        println!("input: {event:?}");
        add_input_to_buffer(&mut buffer, event);
    }
}

fn check_sequence<E: Event + Clone>(mut query: Query<(&Sequence, &ToEvent<E>)>, buffer: Res<SequenceBuffer>, mut writer: EventWriter<E>) {
    if buffer.is_changed() {
        for (sequence, event) in &mut query {
            if buffer.ends_with(sequence) {
                println!("Send event for sequence: {sequence:?}");
                writer.send(event.event.clone());
            }
        }
    }
}

fn update_buffer(query: Query<&Sequence, Or<(Added<Sequence>, Changed<Sequence>)>>,
                 mut buffer: ResMut<SequenceBuffer>) {
    let capacity = query.iter()
        .map(|sequence| sequence.len())
        .max();
    if let Some(size) = capacity {
        println!("Resize sequence buffer: {size}");
        buffer.resize(size);
    }
}

pub fn add_input_to_buffer(buffer: &mut SequenceBuffer, input: InputEvent) {
    buffer.push(input);
}

#[derive(Resource, Default)]
pub struct SequenceBuffer {
    buffer: Vec<InputEvent>,
}

impl SequenceBuffer {
    pub fn resize(&mut self, size: usize) {
        if self.buffer.capacity() != size {
            let mut new_buffer = Vec::with_capacity(size);
            new_buffer.append(&mut self.buffer);
            self.buffer = new_buffer;
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

#[derive(Component, Clone)]
pub struct ToEvent<E: Event + Clone> {
    pub event: E,
}

impl<E: Event + Clone> ToEvent<E> {
    pub fn from_event(event: E) -> Self {
        Self {
            event
        }
    }
}
