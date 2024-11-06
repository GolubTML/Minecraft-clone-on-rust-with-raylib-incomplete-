use raylib::prelude::*;

pub struct Block {
    pub position: Vector3,
    pub color: Color,
}

impl Block {
    pub fn new(x: f32, y: f32, z: f32, color: Color) -> Self {
        Block {
            position: Vector3::new(x, y, z),
            color
        }
    }
}