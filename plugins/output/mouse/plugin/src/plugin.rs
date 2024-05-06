use bevy::app::Update;
use bevy::prelude::{App, Plugin};

use mouse_output_api::{DragAndDrop, MouseClick, MoveMouseRelatively, MoveMouseToPosition, Scroll};

use crate::systems::{on_click_event_system, on_drag_and_drop_event_system, on_move_mouse_relatively_event_system, on_move_mouse_to_position_event_system, on_scroll_event_system};

const MOUSE_OUTPUT_PLUGIN_NAME: &str = "mouse_output";

pub struct MouseOutputPlugin;

impl Plugin for MouseOutputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveMouseRelatively>();
        app.add_event::<MoveMouseToPosition>();
        app.add_event::<Scroll>();
        app.add_event::<MouseClick>();
        app.add_event::<DragAndDrop>();
        app.add_systems(Update, on_move_mouse_relatively_event_system);
        app.add_systems(Update, on_move_mouse_to_position_event_system);
        app.add_systems(Update, on_scroll_event_system);
        app.add_systems(Update, on_click_event_system);
        app.add_systems(Update, on_drag_and_drop_event_system);
    }

    fn name(&self) -> &str {
        MOUSE_OUTPUT_PLUGIN_NAME
    }
}
