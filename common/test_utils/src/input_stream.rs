use std::thread;
use std::thread::JoinHandle;

use crossbeam::channel::{Receiver, unbounded};
use rdev::{EventType, listen};

pub struct InputStream {
    input_channel: Receiver<EventType>,
    _listener_thead: JoinHandle<()>,
}

impl Default for InputStream {
    fn default() -> Self {
        let (s_chan, r_chan) = unbounded();
        let _listener_thead = thread::spawn(move || {
            listen(move |event| {
                if let EventType::MouseMove { .. } = event.event_type {
                    return;
                }
                s_chan
                    .send(event.event_type)
                    .unwrap_or_else(|e| println!("Could not send event {:?}", e));
            }).expect("Could not listen");
        });
        Self {
            _listener_thead,
            input_channel: r_chan,
        }
    }
}

impl Iterator for InputStream {
    type Item = EventType;

    fn next(&mut self) -> Option<Self::Item> {
        self.input_channel.try_recv().ok()
    }
}
