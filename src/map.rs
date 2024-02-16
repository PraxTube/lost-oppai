use bevy::{prelude::*, reflect::Array};
use noisy_bevy::simplex_noise_2d_seeded;

use crate::CHUNK_SIZE;

const SEED: f32 = 69.0;
const NOISE_ZOOM: f32 = 0.04;
const MAP_SIZE: UVec2 = UVec2::new(10, 10);
const EMPTY_INDEX: u8 = 0;

#[derive(Debug, Resource)]
pub struct BitMap {
    pub array: [[u8; (CHUNK_SIZE.y * MAP_SIZE.y) as usize]; (CHUNK_SIZE.x * MAP_SIZE.x) as usize],
}

impl Default for BitMap {
    fn default() -> Self {
        Self {
            array: [[0; (CHUNK_SIZE.y * MAP_SIZE.y) as usize];
                (CHUNK_SIZE.x * MAP_SIZE.x) as usize],
        }
    }
}

impl BitMap {
    fn set(&mut self, u: UVec2, i: u8) {
        self.array[u.x as usize][u.y as usize] = i;
    }

    pub fn get(&self, v: IVec2) -> u8 {
        let u = v + IVec2::new(
            (CHUNK_SIZE.x * MAP_SIZE.x / 2) as i32,
            (CHUNK_SIZE.y * MAP_SIZE.y / 2) as i32,
        );
        if u.x as usize >= self.array.len() || u.y as usize >= self.array[0].len() {
            return EMPTY_INDEX;
        }
        self.array[u.x as usize][u.y as usize]
    }
}

fn generate_chunk(u: UVec2, map: &mut BitMap) {
    let v = Vec2::new(u.x as f32, u.y as f32);
    let noise = simplex_noise_2d_seeded(v * NOISE_ZOOM, SEED);
    let secondary_noise = simplex_noise_2d_seeded(v * NOISE_ZOOM, SEED + 1.0) * 1.0;
    let h = noise + secondary_noise;
    let index = if h < -1.0 {
        1
    } else if h < 0.0 {
        1
    } else {
        0
    };

    map.set(u, index);
}

fn generate_world(mut commands: Commands) {
    let mut map: BitMap = BitMap::default();

    for chunk_pos_x in 0..MAP_SIZE.x {
        for chunk_pos_y in 0..MAP_SIZE.y {
            for x in 0..CHUNK_SIZE.x {
                for y in 0..CHUNK_SIZE.y {
                    let u = UVec2::new(
                        x + chunk_pos_x * CHUNK_SIZE.x,
                        y + chunk_pos_y * CHUNK_SIZE.y,
                    );
                    generate_chunk(u, &mut map);
                }
            }
        }
    }

    commands.insert_resource(map);
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_world);
    }
}
