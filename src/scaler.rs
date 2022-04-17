use raylib::prelude::*;

pub struct Scaler {
    pub scale: f32,
    pub offset: Vector2,
}

impl Scaler {
    pub fn to_screen(&self, v: Vector2) -> Vector2 {
        (v - self.offset) / self.scale
    }

    pub fn to_world(&self, v: Vector2) -> Vector2 {
        (v * self.scale) + self.offset
    }
}