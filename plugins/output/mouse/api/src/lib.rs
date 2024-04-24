use bevy::prelude::Event;

use input_model::Button;

#[derive(Event, Debug, Clone)]
pub struct MoveMouseRelatively {
    pub direction: Direction,
    pub distance: i32,
}

impl MoveMouseRelatively {
    pub fn new(direction: Direction, distance: i32) -> Self {
        Self {
            direction,
            distance,
        }
    }
}

#[derive(Event, Debug)]
pub struct MoveMouseToPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Event, Debug)]
pub struct Scroll {
    pub direction: Direction,
    pub distance: i64,
}

#[derive(Debug, Clone)]
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
