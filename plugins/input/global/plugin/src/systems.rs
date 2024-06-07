use bevy::prelude::{EventReader, EventWriter, Res};

use global_input_api::filter::InputFilterEvent;
use global_input_api::input::InputEvent;
use global_input_api::input_model::filter::InputFilter;

use crate::state::{BLOCKED_EVENTS, GlobalInputState};

pub fn check_input_system(global_input_state: Res<GlobalInputState>, mut input_writer: EventWriter<InputEvent>) {
    if let Ok(input) = global_input_state.input_channel.try_recv() {
        let event = InputEvent(input);
        input_writer.send(event);
    }
}

pub fn on_input_filter_event_system(mut events: EventReader<InputFilterEvent>) {
    for filter in events.read() {
        apply_input_filter(filter);
    }
}

fn apply_input_filter(input_filter: &InputFilterEvent) {
    match &input_filter.0 {
        InputFilter::Block(filter_input) => {
            BLOCKED_EVENTS
                .lock()
                .unwrap()
                .append(&mut filter_input.clone());
        }
        InputFilter::Unblock(filter_input) => {
            BLOCKED_EVENTS
                .lock()
                .unwrap()
                .retain(|event| !filter_input.contains(event));
        }
    }
}
