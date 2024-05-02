use bevy::prelude::Component;

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
    
    pub fn offset(&mut self, x_offset: f32, y_offset: f32){
        self.x_offset(x_offset);
        self.y_offset(y_offset);
    }
    
    pub fn x_offset(&mut self, offset: f32){
        self.x +=offset;
    }

    pub fn y_offset(&mut self, offset: f32){
        self.y +=offset;
    }
}

impl From<(f32, f32)> for NavigationPoint {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}