#![deny(unsafe_code)]

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use rdev::{Event, EventType};

mod poc;
mod core;
mod handlers;
mod translator;

pub fn run_application() {
    let receiver = run_input_listener();
    run_app_loop(receiver);
}

fn run_input_listener() -> Receiver<Event> {
    let (send_channel, receive_channel) = mpsc::channel();
    thread::spawn(move || {
        rdev::listen(move |event| { // todo refactor pattern matching
            let Event { time: _, name: _, event_type } = &event;
            if let EventType::KeyPress(key) | EventType::KeyRelease(key) = event_type { // todo remove this filtering
                send_channel
                    .send(event)
                    .unwrap_or_else(|e| println!("Could not send event {:?}", e));
            }
        }).expect("Could not listen");
    });
    receive_channel
}

fn run_app_loop(input_receiver: Receiver<Event>) {
    let handlers_registry = handlers::Registry::default();
    let state = core::State {};
    let mut executor = core::Executor::new(handlers_registry.get_handlers(), state);
    let mut translator = translator::Translator{};
    loop {
        if let Ok(event) = input_receiver.try_recv() {
            let labels = translator.translate(event);
            labels.iter()
                .for_each(|label| executor.run(label))
        }
    }
}
