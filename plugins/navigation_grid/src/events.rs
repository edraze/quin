use bevy::prelude::{Event};

#[derive(Event, Clone)]
pub struct ActivateNavigationGrid;

#[derive(Event, Clone)]
pub struct DeactivateNavigationGrid;

#[derive(Event, Clone, Eq, PartialEq, Debug)]
pub struct NavigateToLabel{
    label: String
}

impl NavigateToLabel {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string()
        }
    }
}
