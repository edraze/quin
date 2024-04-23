use bevy::prelude::Event;

use crate::plugins::global_input::events::Button;

#[derive(Event, Debug)]
pub struct MoveMouseRelatively {
    pub direction: Direction,
    pub distance: i32,
}

#[derive(Event, Debug)]
pub struct MoveMouseToPosition {
    pub x: f64,
    pub y: f64
}

#[derive(Event, Debug)]
pub struct Scroll {
    pub direction: Direction,
    pub distance: i64,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Event, Debug)]
pub struct MouseClick {
    // todo move keyboard & mouse model to common module
    pub button: Button,
}

#[derive(Event, Debug)]
pub struct DragAndDrop {
    pub action: DragAndDropAction,
    pub button: Button,
}

#[derive(Debug)]
pub enum DragAndDropAction {
    Start,
    End,
}