use raylib::prelude::*;

pub struct Scaler {
    pub scale: f32,
    pub x_offset: f32,
    pub y_offset: f32,
}

impl Scaler {
    pub fn _to_screen(&self, x: f32, y: f32) -> (i32, i32) {
        (
            ((x + self.x_offset) / self.scale) as i32,
            ((y + self.y_offset) / self.scale) as i32,
        )
    }

    pub fn to_world(&self, x: i32, y: i32) -> (f32, f32) {
        (
            (x as f32 * self.scale) - self.x_offset,// - (crate::WIDTH/2) as f32,
            (y as f32 * self.scale) - self.y_offset,// - (crate::WIDTH/2) as f32,
        )
    }

    pub fn to_screen_v(&self, v: Vector2) -> Vector2 {
        let offset = Vector2 {x: self.x_offset as f32, y: self.y_offset as f32};
        
        (v - offset) / self.scale
    }

    pub fn to_world_v(&self, v: Vector2) -> Vector2 {
        let offset = Vector2 {x: self.x_offset as f32, y: self.y_offset as f32};
        
        (v * self.scale) + offset// - (crate::WIDTH/2) as f32
    }
}