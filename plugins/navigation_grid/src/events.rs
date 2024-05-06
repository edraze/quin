use bevy::prelude::{Event};

#[derive(Event, Clone)]
pub struct ActivateNavigationGrid;

#[derive(Event, Clone)]
pub struct DeactivateNavigationGrid;

#[derive(Event, Clone)]
pub struct ActivateMainGrid;

#[derive(Event, Clone)]
pub struct DeactivateMainGrid;

#[derive(Event, Clone)]
pub struct ActivateSubGrid;

#[derive(Event, Clone)]
pub struct DeactivateSubGrid;

#[derive(Event, Clone, Eq, PartialEq, Debug)]
pub struct NavigateToLabel{
    pub label: String
}

impl NavigateToLabel {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string()
        }
    }
}

#[derive(Event, Clone, Eq, PartialEq, Debug)]
pub struct NavigateToSubLabel{
    pub label: String
}

impl NavigateToSubLabel {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string()
        }
    }
}

#[derive(Event)]
pub struct UpdateSubGridPosition{
    pub x: f32,
    pub y: f32
}

impl UpdateSubGridPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }
}
