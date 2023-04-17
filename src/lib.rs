#![deny(unsafe_code)]

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use rdev::{Event, EventType};

mod poc; // todo remove
mod core;
mod common;
mod registry;
mod translator;
mod config;
mod overlay;

pub fn run_application() {
    let receiver = run_input_listener();
    run_app_loop(receiver);
}

fn run_input_listener() -> Receiver<Event> {
    let (send_channel, receive_channel) = mpsc::channel();
    thread::spawn(move || {
        rdev::listen(move |event| { // todo refactor pattern matching
            let Event { time: _, name: _, event_type } = &event;
            if let EventType::KeyPress(_) | EventType::KeyRelease(_) = event_type { // todo remove this filtering
                send_channel
                    .send(event)
                    .unwrap_or_else(|e| println!("Could not send event {:?}", e));
            }
        }).expect("Could not listen");
    });
    receive_channel
}

fn run_app_loop(input_receiver: Receiver<Event>) {
    let state = core::State {};
    let handlers = registry::get_handlers();
    let mut executor = core::Executor::new(handlers, state);
    let mut translator = translator::Translator::default();

    overlay::run(move |gui_ctx| {
        if let Ok(event) = input_receiver.try_recv() {
            let labels = translator.translate(event);
            labels.iter()
                .for_each(|label| executor.run(label))
        }
        executor.draw(gui_ctx);
    });
}
