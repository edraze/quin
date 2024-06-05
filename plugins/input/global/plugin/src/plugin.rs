use bevy::app::{App, Plugin, Update};

use global_input_api::filter::InputFilterEvent;
use global_input_api::input::InputEvent;

use crate::state::GlobalInputState;
use crate::systems::{check_input_system, on_input_filter_event_system};

const GLOBAL_INPUT_PLUGIN_NAME: &str = "global_input";

#[derive(Eq, PartialEq)]
pub struct GlobalInputPlugin;

impl Plugin for GlobalInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GlobalInputState>()
            .add_event::<InputEvent>()
            .add_event::<InputFilterEvent>()
            .add_systems(Update, check_input_system)
            .add_systems(Update, on_input_filter_event_system);
    }

    fn name(&self) -> &str {
        GLOBAL_INPUT_PLUGIN_NAME
    }
}
