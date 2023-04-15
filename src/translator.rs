use rdev::{Event, Key};
use rdev::EventType::KeyRelease;
use crate::core::Labels;

pub struct Translator {

}

impl Translator {
    pub fn translate(&mut self, event: Event) -> Vec<Labels> {
        let mut labels = Vec::new();

        if let KeyRelease(Key::AltGr) = event.event_type {
            labels.push(Labels::Keys(String::from("gm_activate")));
        }

        labels
    }
}
