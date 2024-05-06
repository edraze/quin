use bevy::prelude::{EventReader, EventWriter, Res};
use global_input_api::filter::InputFilterEvent;
use global_input_api::input::InputEvent;
use crate::state::{BLOCKED_EVENTS, GlobalInputState};

pub fn global_input_handler_system(global_input_state: Res<GlobalInputState>, mut input_event: EventWriter<InputEvent>) {
    if let Ok(event) = global_input_state.input_channel.try_recv() {
        input_event.send(event);
    }
}

pub fn handle_input_filter_event_system(mut events: EventReader<InputFilterEvent>) {
    for filter in events.read() {
        apply_input_filter(filter);
    }
}

fn apply_input_filter(input_filter: &InputFilterEvent) {
    match input_filter {
        InputFilterEvent::Block(filter_input) => {
            let mut block_events = filter_input.to_input_events();
            BLOCKED_EVENTS
                .lock()
                .unwrap()
                .append(&mut block_events);
        }
        InputFilterEvent::Unblock(filter_input) => {
            let unblock_events = filter_input.to_input_events();
            BLOCKED_EVENTS
                .lock()
                .unwrap()
                .retain(|event| !unblock_events.contains(event));
        }
    }
}
