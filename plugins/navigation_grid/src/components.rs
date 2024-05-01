use bevy::prelude::Component;

#[derive(Component)]
pub struct NavigationPoint {
    pub x: f64,
    pub y: f64,
}

impl NavigationPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl From<(f64, f64)> for NavigationPoint {
    fn from((x, y): (f64, f64)) -> Self {
        Self::new(x, y)
    }
}