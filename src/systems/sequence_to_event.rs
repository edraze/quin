use bevy::prelude::{Event, EventReader, EventWriter};

use crate::plugins::input_sequence::events::Sequence;

// todo may be it will be better to not create this system for every E
pub fn sequence_to_event<E>(target_sequence: Sequence) -> impl Fn(EventReader<Sequence>, EventWriter<E>)
    where E: Event + Default {
    move |sequence_events: EventReader<Sequence>, event_writer: EventWriter<E>| {
        sequence_to_event_system(sequence_events, event_writer, &target_sequence);
    }
}

fn sequence_to_event_system<E>(mut sequence_events: EventReader<Sequence>, mut event_writer: EventWriter<E>, target_sequence: &Sequence)
    where E: Event + Default {
    sequence_events
        .read()
        .for_each(|sequence| {
            if target_sequence.eq(sequence) {
                event_writer.send(Default::default());
            }
        });
}
