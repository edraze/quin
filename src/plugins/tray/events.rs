use bevy::prelude::Event;

#[derive(Event)]
pub struct TrayClick;

#[derive(Event)]
pub struct TrayItemClick {
    pub id: String
}

impl TrayItemClick {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string()
        }
    }
}

#[derive(Event, Debug)]
pub struct CreateTrayItem {
    pub id: String
}

impl CreateTrayItem {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string()
        }
    }
}