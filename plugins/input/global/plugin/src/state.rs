use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::thread::LocalKey;

use bevy::prelude::{error, Resource};
use crossbeam::channel::{bounded, Receiver, Sender, unbounded};
use once_cell::sync::Lazy;
use rdev::{Event, EventType};

use global_input_api::input_model::{ButtonInput, DeviceInput, Input, KeyboardInput, Modified, Modifier, MouseInput};

// todo remove mutex here to improve overall fps
pub static BLOCKED_EVENTS: Lazy<Arc<Mutex<Vec<Input>>>> = Lazy::new(|| { Arc::new(Mutex::new(Vec::new())) });
static MODIFIERS: Lazy<Mutex<HashSet<Modifier>>> = Lazy::new(|| { Mutex::new(HashSet::new()) });

// todo hard to kill rdev listener because https://github.com/Narsil/rdev/issues/72
// todo if sub thread not drop it creates many threads in tests
#[derive(Resource)]
pub struct GlobalInputState {
    pub input_channel: Receiver<Input>,
    _grabbing_thead: JoinHandle<()>,
}

impl Default for GlobalInputState {
    fn default() -> Self {
        let (input_chan_s, input_chan_r) = unbounded();
        let _grabbing_thead = thread::spawn(move || grab(input_chan_s));
        Self {
            input_channel: input_chan_r,
            _grabbing_thead,
        }
    }
}

// todo remove panics
fn grab(s_chan: Sender<Input>) {
    rdev::grab(move |raw_event| {
        if is_ignored(&raw_event) {
            return Some(raw_event);
        }
        let event = modify((&raw_event).into());
        send_input(&event, &s_chan);

        let blocked_events = BLOCKED_EVENTS.lock().unwrap();
        if is_blocked(blocked_events.as_slice(), &event) {
            println!("Global input event blocked: {event:?}");
            None
        } else {
            Some(raw_event)
        }
    }).expect("Could not listen");
}

fn is_ignored(raw_event: &Event) -> bool {
    if let EventType::MouseMove { .. } = raw_event.event_type {
        return true;
    }
    false
}

// todo fix bug when modifier release should send device input but send modified input
fn modify(device_input: DeviceInput) -> Input {
    let mut modifiers = MODIFIERS.lock().unwrap();
    let actual_modifiers = modifiers.clone();
    if let DeviceInput::Keyboard(KeyboardInput::Pressed(key)) = device_input {
        modifiers.insert(key.into());
    } else if let DeviceInput::Mouse(MouseInput::Button(ButtonInput::Pressed(button))) = device_input {
        modifiers.insert(button.into());
    } else if let DeviceInput::Keyboard(KeyboardInput::Released(key)) = device_input {
        modifiers.retain(|modifier| !modifier.eq(&key.into()));
    } else if let DeviceInput::Mouse(MouseInput::Button(ButtonInput::Released(button))) = device_input {
        modifiers.retain(|modifier| !modifier.eq(&button.into()));
    }
    if actual_modifiers.is_empty() || modifiers.is_empty() {
        Input::Device(device_input)
    } else {
        let mut modifiers: Vec<_> = actual_modifiers.into_iter().collect();
        modifiers.reverse();
        Input::Modified(Modified {
            modifiers,
            input: Box::new(Input::Device(device_input)),
        })
    }
}

fn send_input(event: &Input, s_chan: &Sender<Input>) {
    s_chan
        .send(event.clone())
        .unwrap_or_else(|e| error!("Could not send event {:?}", e));
}

fn is_blocked(blocked_events: &[Input], event: &Input) -> bool {
    blocked_events.contains(event)
}
