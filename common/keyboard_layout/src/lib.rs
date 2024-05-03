use std::any::type_name;
use std::marker::PhantomData;

use bevy::app::App;
use bevy::prelude::Event;

use input_sequence_plugin::{listen_sequences, SequencesToEvent};
use toggle::{add_toggle, add_toggle_event};

pub struct Layout<'a, A: Event + Clone, D: Event + Clone> {
    app: &'a mut App,
    _activation_event: PhantomData<A>,
    _deactivation_event: PhantomData<D>,
    event_type_names: Vec<String>,
}

impl<'a, A: Event + Clone, D: Event + Clone> Layout<'a, A, D> {
    pub fn new(app: &'a mut App) -> Self {
        add_toggle::<A, D>(app);
        Self {
            app,
            _activation_event: PhantomData,
            _deactivation_event: PhantomData,
            event_type_names: Default::default()
        }
    }

    pub fn bind<E: Event + Clone>(mut self, binding: impl Into<SequencesToEvent<E>>) -> Self {
        listen_sequences(self.app, binding.into());
        let event_type_name = type_name::<E>().to_string();
        if !self.event_type_names.contains(&event_type_name) {
            add_toggle_event::<A, D, E>(self.app);
            self.event_type_names.push(event_type_name);
        }
        self
    }
}


