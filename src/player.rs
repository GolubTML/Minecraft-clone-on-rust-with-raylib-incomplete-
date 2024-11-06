use raylib::prelude::*;

use crate::block::Block;

pub struct Player {
    pub velocity: Vector3,
    pub position: Vector3, 
    pub camera: Camera3D,
    pub jump_velocity: f32,
    pub gravity: f32,
    pub on_ground: bool,
} 

impl Player {
    pub fn new(pos: Vector3) -> Self {
        Self {
            position: pos,
            camera: Camera3D::perspective(
                pos,
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(0.0, 1.0, 0.0),
                80.0,
            ),
            velocity: Vector3::zero(),
            jump_velocity: 8.0,
            gravity: -9.81,
            on_ground: false,
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, delta_time: f32, block: &[Block]) {
        let mut movement = Vector3::zero();

        let forward = Vector3::new(self.camera.target.x - self.camera.position.x, 0.0, self.camera.target.z - self.camera.position.z);
        let right = Vector3::cross(&forward, self.camera.up).normalized();

        if rl.is_key_down(KeyboardKey::KEY_W) {
            movement += forward;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            movement -= forward;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            movement -= right;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            movement += right;
        }

        if self.on_ground {
            if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                self.velocity.y += self.jump_velocity;
                self.on_ground = false;
            } else {
                self.velocity.y = 0.0;
            }
        } else {
            self.velocity.y += self.gravity * delta_time;
        }

        self.position += movement * delta_time;
        self.position.y += self.velocity.y * delta_time;

        self.on_ground = self.check_collision(&block);

        self.camera.position = self.position;
    }

    fn check_collision(&mut self, blocks: &[Block]) -> bool {
        for block in blocks {
            if self.position.y <= block.position.y + 1.5 && 
            self.position.y >= block.position.y - 0.5 &&
            self.position.x > block.position.x - 0.5 && 
            self.position.x < block.position.x + 0.5 &&
            self.position.z > block.position.z - 0.5 && 
            self.position.z < block.position.z + 0.5 {
                self.position.y = block.position.y + 1.5;
                return true;
            }
        }

        false
    }// complete now
}