use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use bevy::app::{App, Plugin, Update};
use bevy::prelude::{error, EventReader, EventWriter, Res, Resource};
use crossbeam::channel::Receiver;
use crossbeam::channel::unbounded;
use once_cell::sync::Lazy;
use rdev::{EventType, grab};
use global_input_api::filter::InputFilterEvent;
use global_input_api::input::InputEvent;

// emitter

const GLOBAL_INPUT_PLUGIN_NAME: &str = "global_input";

// todo divide core
pub struct GlobalInputPlugin;

impl Plugin for GlobalInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GlobalInputState>()
            .add_event::<InputEvent>()
            .add_event::<InputFilterEvent>()
            .add_systems(Update, global_input_handler)
            .add_systems(Update, handle_input_filter_event);
    }

    fn name(&self) -> &str {
        GLOBAL_INPUT_PLUGIN_NAME
    }
}

#[derive(Resource)]
pub struct GlobalInputState {
    // todo drop thread on plugin reload
    input_channel: Receiver<InputEvent>,
    _grabbing_thead: JoinHandle<()>,
}

static BLOCKED_EVENTS: Lazy<Arc<Mutex<Vec<InputEvent>>>> = Lazy::new(|| { Arc::new(Mutex::new(Vec::new())) });

fn is_event_blocked(blocked_events: &[InputEvent], event: &InputEvent) -> bool {
    if let InputEvent::MouseMove { .. } = event {
        blocked_events.contains(&InputEvent::MouseMove { x: 0., y: 0. })
    } else {
        blocked_events.contains(event)
    }
}

impl Default for GlobalInputState {
    fn default() -> Self {
        let (s_chan, r_chan) = unbounded();
        let blocked_events = BLOCKED_EVENTS.clone();
        let _grabbing_thead = thread::spawn(move || {
            grab(move |raw_event| {
                // todo filter mouse move events by time delta, send event every second
            if let EventType::MouseMove { .. } = raw_event.event_type {
                    return Some(raw_event);
                }
                let event: InputEvent = (&raw_event).into();
                let blocked_events = blocked_events.lock().unwrap();
                s_chan
                    .send(event.clone())
                    .unwrap_or_else(|e| error!("Could not send event {:?}", e));
                if is_event_blocked(blocked_events.as_slice(), &event) {
                    println!("Global input event blocked: {event:?}");
                    None
                } else {
                    Some(raw_event)
                }
            }).expect("Could not listen");
        });
        Self {
            _grabbing_thead,
            input_channel: r_chan,
        }
    }
}

pub fn global_input_handler(global_input_state: Res<GlobalInputState>, mut input_event: EventWriter<InputEvent>) {
    if let Ok(event) = global_input_state.input_channel.try_recv() {
        input_event.send(event);
    }
}

pub fn handle_input_filter_event(mut events: EventReader<InputFilterEvent>) {
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
