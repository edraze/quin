use bevy::prelude::{Added, Changed, DetectChanges, Event, EventReader, EventWriter, Or, Query, Res, ResMut};
use global_input_api::input::InputEvent;
use input_sequence_api::{ResetSequenceBuffer, Sequence, ToEvent};
use crate::state::SequenceBuffer;

pub fn handle_input_event_system(mut buffer: ResMut<SequenceBuffer>, mut events: EventReader<InputEvent>) {
    for event in events.read().cloned() {
        println!("input: {event:?}");
        add_input_to_buffer(&mut buffer, event);
    }
}

pub fn check_sequence_system<E: Event + Clone>(mut query: Query<(&Sequence, &ToEvent<E>)>, buffer: Res<SequenceBuffer>, mut writer: EventWriter<E>) {
    if buffer.is_changed() {
        for (sequence, event) in &mut query {
            if buffer.ends_with(sequence) {
                println!("Send event for sequence: {sequence:?}");
                writer.send(event.event.clone());
            }
        }
    }
}

pub fn update_buffer_system(query: Query<&Sequence, Or<(Added<Sequence>, Changed<Sequence>)>>,
                        mut buffer: ResMut<SequenceBuffer>) {
    let capacity = query.iter()
        .map(|sequence| sequence.len())
        .max();
    if let Some(size) = capacity {
        println!("Resize sequence buffer: {size}");
        buffer.resize(size);
    }
}

pub fn reset_sequence_buffer_system(mut reset_sequence_buffer_reader: EventReader<ResetSequenceBuffer>,
                                mut buffer: ResMut<SequenceBuffer>){
    if reset_sequence_buffer_reader.read().count() > 0 {
        println!("reset sequence buffer");
        buffer.reset();
    }
}

pub fn add_input_to_buffer(buffer: &mut SequenceBuffer, input: InputEvent) {
    buffer.push(input);
}