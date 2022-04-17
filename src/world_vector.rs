use crate::VECTOR_ACCELERATION;
use crate::scaler::Scaler;
use raylib::prelude::*;

pub struct WorldVector {
    pub root: Vector2,
    pub velocity: Vector2,
}

impl WorldVector {
    pub fn new(root: Vector2, velocity: Vector2) -> WorldVector {
        WorldVector {
            root: root,
            velocity: velocity,
        }
    }

    pub fn update(&mut self) {
        self.root += self.velocity * VECTOR_ACCELERATION;
    }
}