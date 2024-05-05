use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct KeyboardToMouseState {
    pub drag_and_drop_active: bool
}