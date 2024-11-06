#[warn(unused_imports)]
use crate::block::Block;
use noise::{NoiseFn, Perlin};
use raylib::prelude::*;

pub fn gen_world(dx: i32, dy: i32, dz: i32, grid_size: f32) -> Vec<Block> {
    let mut blocks = Vec::new();

    let perlin = Perlin::new(20);

    for x in 0..dx {
        for y in 0..dy {
            for z in 0..dz {
                let nx = x as f64 * 0.1;
                let ny = y as f64 * 0.1;
                let nz = z as f64 * 0.1;
                let noise_value = perlin.get([nx, ny, nz]);

                println!("Noise value at ({}, {}, {}): {}", x, y, z, noise_value);

                let block_type = if noise_value > 0.5 {
                    //Dirt
                    Block::new(
                        x as f32 * grid_size, 
                        y as f32 * grid_size, 
                        z as f32 * grid_size, 
                        Color::GREEN
                    )
                } else if noise_value > 0.3 {
                    //Stone
                    Block::new(
                        x as f32 * grid_size, 
                        y as f32 * grid_size, 
                        z as f32 * grid_size, 
                        Color::GRAY
                    )
                } else {
                    //Air
                    continue;
                };

                blocks.push(block_type);
            }
        }
    }

    println!("Generated {} blocks", blocks.len());

    blocks
}