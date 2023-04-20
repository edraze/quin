use std::collections::{HashMap, HashSet};
use crate::config::ConfigLoader;
use crate::core::{Event, Label};
use crate::registry;

pub struct Translator {
    bindings: Vec<(String, String)>,
    input_buffer: Vec<String>,
}

impl Default for Translator {
    fn default() -> Self {
        let config = ConfigLoader::load_default();
        let input_buffer = Vec::with_capacity(8); // todo determinate capacity by max value from config

        let mut bindings: HashSet<(String, String)> = registry::get_bindings().into_iter()
            .map(|binding| (binding.label, binding.default_input))
            .collect();
        bindings.extend(config.bindings.into_iter().collect::<Vec<_>>());

        let mut bindings = Vec::from_iter(bindings);
        bindings.sort_by_key(|(_, input_buffer)| input_buffer.len());
        Translator { bindings, input_buffer }
    }
}

impl Translator {
    pub fn translate(&mut self, event: Event) -> Vec<Label> {
        self.save_input(event.to_string());
        let input_buffer_string = self.input_buffer.join("");

        self.bindings.iter()
            .filter(|(label, hotkeys)| input_buffer_string.ends_with(hotkeys))
            .map(|(label, hotkeys)| Label::Keys(label.clone()))
            .collect()
    }

    fn save_input(&mut self, input: String) {
        if self.input_buffer.capacity() != 0 {
            if self.input_buffer.len() == self.input_buffer.capacity() {
                self.input_buffer.remove(0);
            }
            self.input_buffer.push(input);
        }
    }
}
