use bevy::prelude::Resource;
use global_input_api::input::InputEvent;
use input_sequence_api::Sequence;

#[derive(Resource, Default)]
pub struct SequenceBuffer {
    buffer: Vec<InputEvent>,
}

impl SequenceBuffer {
    pub fn resize(&mut self, size: usize) {
        if self.buffer.capacity() != size {
            let mut new_buffer = Vec::with_capacity(size);
            new_buffer.append(&mut self.buffer);
            self.buffer = new_buffer;
        }
    }

    pub fn push(&mut self, input: InputEvent) {
        if self.buffer.capacity() != 0 {
            if self.buffer.len() == self.buffer.capacity() {
                self.buffer.remove(0);
            }
            self.buffer.push(input);
        }
    }

    pub fn ends_with(&self, sequence: &Sequence) -> bool {
        self.buffer.ends_with(&sequence.input_events)
    }

    pub fn reset(&mut self){
        self.buffer.clear();
    }
}