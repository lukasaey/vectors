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

    pub fn update(&mut self, dt: &f32) {
        self.root += self.velocity * *dt;
    }
}