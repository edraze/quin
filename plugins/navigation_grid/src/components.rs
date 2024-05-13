use bevy::math::Vec2;
use bevy::prelude::{Component, Resource};

#[derive(Component)]
pub struct NavigationPoint {
    pub x: f32,
    pub y: f32,
}

impl NavigationPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn offset(&mut self, x_offset: f32, y_offset: f32) {
        self.x_offset(x_offset);
        self.y_offset(y_offset);
    }

    pub fn x_offset(&mut self, offset: f32) {
        self.x += offset;
    }

    pub fn y_offset(&mut self, offset: f32) {
        self.y += offset;
    }
}

impl From<(f32, f32)> for NavigationPoint {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

#[derive(Resource)]
pub struct SubGrid {
    pub center: Vec2,
}

impl SubGrid {
    pub fn new(x_center: f32, y_center: f32) -> Self {
        Self {
            center: (x_center, y_center).into(),
        }
    }
}
