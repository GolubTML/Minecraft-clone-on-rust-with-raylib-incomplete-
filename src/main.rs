mod world;
mod block;
mod player;

use raylib::prelude::*;
use world::gen_world;
use player::Player;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1600, 800)
        .title("Minecraft Clone")
        .build();

    rl.set_target_fps(120);

    let mut player = Player::new(Vector3::new(10.0, 1000.0, 10.0));
    let world = gen_world(32, 64, 32, 1.0);

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();

        player.update(&mut rl, delta_time, &world);
        rl.update_camera(&mut player.camera, CameraMode::CAMERA_FIRST_PERSON);
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        {
            let mut d = d.begin_mode3D(&player.camera);
            
            for block in &world {
                d.draw_cube(block.position, 1.0, 1.0, 1.0, block.color);
                d.draw_cube_wires(block.position, 1.0, 1.0, 1.0, Color::BLACK);
            }
        }

        d.draw_fps(10, 10);
    }
}