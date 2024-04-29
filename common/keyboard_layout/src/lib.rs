use std::marker::PhantomData;

use bevy::app::App;
use bevy::prelude::Event;

use input_sequence_plugin::{listen_sequences, SequencesToEvent};
use toggle::{add_toggle, add_toggle_event};

pub struct Layout<'a, A: Event + Clone, D: Event + Clone> {
    app: &'a mut App,
    _activation_event: PhantomData<A>,
    _deactivation_event: PhantomData<D>,
}

impl<'a, A: Event + Clone, D: Event + Clone> Layout<'a, A, D> {
    pub fn new(app: &'a mut App,
               activation_binding: SequencesToEvent<A>,
               deactivation_binding: SequencesToEvent<D>) -> Self {
        listen_sequences(app, activation_binding);
        listen_sequences(app, deactivation_binding);
        add_toggle::<A, D>(app);
        Self {
            app,
            _activation_event: PhantomData,
            _deactivation_event: PhantomData,
        }
    }

    pub fn bind<E: Event + Clone>(self, binding: impl Into<SequencesToEvent<E>>) -> Self {
        listen_sequences(self.app, binding.into());
        add_toggle_event::<A, D, E>(self.app);
        self
    }
}


