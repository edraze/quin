use bevy::app::{App, Plugin};
use bevy::prelude::{Event, Events, Update, World};

use global_input_plugin::GlobalInputPlugin;
use input_sequence_api::{ResetSequenceBuffer, SequenceComponent, ToEvent};
pub use input_sequence_api::SequencesToEvent;

use crate::state::SequenceBuffer;
use crate::systems::{check_sequence_system, handle_input_event_system, reset_sequence_buffer_system, update_buffer_system};

const INPUT_SEQUENCE_PLUGIN_NAME: &str = "input_sequence";

pub struct InputSequencePlugin;

impl Plugin for InputSequencePlugin {
    fn build(&self, app: &mut App) {
        if !app.get_added_plugins().contains(&&GlobalInputPlugin) {
            app.add_plugins(GlobalInputPlugin);
        }
        app.init_resource::<SequenceBuffer>();
        app.add_event::<ResetSequenceBuffer>();
        app.add_systems(Update, handle_input_event_system);
        app.add_systems(Update, update_buffer_system);
        app.add_systems(Update, reset_sequence_buffer_system);
    }

    fn name(&self) -> &str {
        INPUT_SEQUENCE_PLUGIN_NAME
    }
}

pub fn listen_sequences<E: Event + Clone>(app: &mut App, binding: impl Into<SequencesToEvent<E>>) {
    if !app.world.contains_resource::<Events<E>>() {
        app.add_event::<E>();
        app.add_systems(Update, check_sequence_system::<E>);
    }
    let sequence_to_event = binding.into();
    for sequence in sequence_to_event.sequences {
        subscribe(&mut app.world, sequence, sequence_to_event.event.clone());
    }
}

pub fn subscribe<E: Event + Clone>(world: &mut World, sequence: SequenceComponent, event: E) {
    world.spawn((sequence, ToEvent::from_event(event)));
}
