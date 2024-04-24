// middleware/handler

use bevy::app::{App, Plugin, Update};
use bevy::prelude::{EventReader, EventWriter};
use global_input_api::InputEvent;
use gui_plugin::GuiEvent;


const INPUT_TO_GUI_PLUGIN_NAME: &str = "input_to_gui";

pub struct InputToGuiPlugin;

impl Plugin for InputToGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, input_to_gui_event);
    }

    fn name(&self) -> &str {
        INPUT_TO_GUI_PLUGIN_NAME
    }
}
fn input_to_gui_event(mut input_events: EventReader<InputEvent>, mut gui_event: EventWriter<GuiEvent>) {
    for input_event in input_events.read() {
        gui_event.send(GuiEvent::DrawLabel(format!("{input_event:?}")));
    }
}
