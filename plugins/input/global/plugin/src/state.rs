use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use bevy::prelude::{error, Resource};
use crossbeam::channel::{Receiver, unbounded};
use once_cell::sync::Lazy;
use rdev::{EventType, grab};

use global_input_api::input::InputEvent;

pub static BLOCKED_EVENTS: Lazy<Arc<Mutex<Vec<InputEvent>>>> = Lazy::new(|| { Arc::new(Mutex::new(Vec::new())) });

#[derive(Resource)]
pub struct GlobalInputState {
    // todo drop thread on plugin reload
    pub input_channel: Receiver<InputEvent>,
    _grabbing_thead: JoinHandle<()>,
}

impl Default for GlobalInputState {
    fn default() -> Self {
        let (s_chan, r_chan) = unbounded();
        let blocked_events = BLOCKED_EVENTS.clone();
        let _grabbing_thead = thread::spawn(move || {
            grab(move |raw_event| {
                // todo filter mouse move events by time delta
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

fn is_event_blocked(blocked_events: &[InputEvent], event: &InputEvent) -> bool {
    if let InputEvent::MouseMove { .. } = event {
        blocked_events.contains(&InputEvent::MouseMove { x: 0., y: 0. })
    } else {
        blocked_events.contains(event)
    }
}
