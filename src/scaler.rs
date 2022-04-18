use crate::WorldVector;
use raylib::prelude::*;

pub struct Scaler {
    pub scale: f32,
    pub offset: Vector2,
}

#[allow(dead_code)]
impl Scaler {
    pub fn to_screen(&self, v: &Vector2) -> Vector2 {
        (*v - self.offset) / self.scale
    }

    pub fn to_world(&self, v: &Vector2) -> Vector2 {
        (*v * self.scale) + self.offset
    }

    pub fn to_screen_v(&self, v: &WorldVector) -> WorldVector {
        WorldVector::new(self.to_screen(&v.root), v.velocity / self.scale)
    }

    pub fn to_world_v(&self, v: &WorldVector) -> WorldVector {
        WorldVector::new(self.to_world(&v.root), v.velocity * self.scale)
    }
}

