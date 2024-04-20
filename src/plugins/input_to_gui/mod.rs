// middleware/handler

use bevy::prelude::{EventReader, EventWriter};

use crate::plugins::global_input::events::InputEvent;
use crate::plugins::gui::GuiEvent;

pub fn input_to_gui_event(mut input_events: EventReader<InputEvent>, mut gui_event: EventWriter<GuiEvent>) {
    for input_event in input_events.read() {
        gui_event.send(GuiEvent::DrawLabel(format!("{input_event:?}")));
    }
}
