use bevy::{prelude::*, reflect::Array};
use noisy_bevy::simplex_noise_2d_seeded;

use crate::CHUNK_SIZE;

const SEED: f32 = 69.0;
const NOISE_ZOOM: f32 = 0.04;
const MAP_SIZE: UVec2 = UVec2::new(10, 10);
const WATER: u8 = 1;
const EMPTY_INDEX: u8 = WATER;

#[derive(Debug, Resource)]
pub struct BitMap {
    water: [[bool; (CHUNK_SIZE.y * MAP_SIZE.y) as usize]; (CHUNK_SIZE.x * MAP_SIZE.x) as usize],
    grass: [[u8; (CHUNK_SIZE.y * MAP_SIZE.y) as usize]; (CHUNK_SIZE.x * MAP_SIZE.x) as usize],
}

impl Default for BitMap {
    fn default() -> Self {
        Self {
            water: [[true; (CHUNK_SIZE.y * MAP_SIZE.y) as usize];
                (CHUNK_SIZE.x * MAP_SIZE.x) as usize],
            grass: [[WATER; (CHUNK_SIZE.y * MAP_SIZE.y) as usize];
                (CHUNK_SIZE.x * MAP_SIZE.x) as usize],
        }
    }
}

impl BitMap {
    /// All the mask matrices should always have the same size
    fn in_bounds(&self, u: UVec2) -> bool {
        let max_x = self.water.len() as u32 - 1;
        let max_y = self.water[0].len() as u32 - 1;

        u.x < max_x && u.y < max_y
    }

    fn get_water(&self, u: UVec2) -> bool {
        if u.x as usize >= self.grass.len() || u.y as usize >= self.grass[0].len() {
            return false;
        }
        self.water[u.x as usize][u.y as usize]
    }

    fn set_water(&mut self, u: UVec2, b: bool) {
        self.water[u.x as usize][u.y as usize] = b;
    }

    fn set_grass(&mut self, u: UVec2, i: u8) {
        self.grass[u.x as usize][u.y as usize] = i;
    }

    pub fn get(&self, v: IVec2) -> u8 {
        let u = v + IVec2::new(
            (CHUNK_SIZE.x * MAP_SIZE.x / 2) as i32,
            (CHUNK_SIZE.y * MAP_SIZE.y / 2) as i32,
        );
        if u.x as usize >= self.grass.len() || u.y as usize >= self.grass[0].len() {
            return EMPTY_INDEX;
        }
        self.grass[u.x as usize][u.y as usize]
    }
}

fn generate_water(map: &mut BitMap) {
    fn g_water(u: UVec2, map: &mut BitMap) {
        if u.x == 0 || u.y == 0 {
            return;
        }
        if u.x == map.water.len() as u32 - 1 || u.y == map.water[0].len() as u32 - 1 {
            return;
        }

        let v = Vec2::new(u.x as f32, u.y as f32);

        let noise = simplex_noise_2d_seeded(v * NOISE_ZOOM, SEED);
        let secondary_noise = simplex_noise_2d_seeded(v * NOISE_ZOOM, SEED + 1.0) * 1.0;
        let h = noise + secondary_noise;

        let b = if h < 0.0 { true } else { false };
        map.set_water(u, b);
    }

    for c_x in 0..MAP_SIZE.x {
        for c_y in 0..MAP_SIZE.y {
            for x in 0..CHUNK_SIZE.x {
                for y in 0..CHUNK_SIZE.y {
                    let u = UVec2::new(x + c_x * CHUNK_SIZE.x, y + c_y * CHUNK_SIZE.y);
                    g_water(u, map);
                }
            }
        }
    }
}

fn filter_water(map: &mut BitMap) {
    fn single_tile(u: UVec2, map: &BitMap) -> bool {
        if !map.in_bounds(u) {
            return false;
        }
        if u.x == 0 || u.y == 0 {
            return false;
        }

        let is_water = map.get_water(u);
        let horizontal_count = (map.get_water(u + UVec2::new(1, 0)) == is_water) as u32
            + (map.get_water(u - UVec2::new(1, 0)) == is_water) as u32;
        let vertical_count = (map.get_water(u + UVec2::new(0, 1)) == is_water) as u32
            + (map.get_water(u - UVec2::new(0, 1)) == is_water) as u32;

        horizontal_count == 0 || vertical_count == 0
    }

    fn f_water(u: UVec2, map: &mut BitMap) {
        if !single_tile(u, map) {
            return;
        }

        map.set_water(u, !map.get_water(u));
    }

    for c_x in 0..MAP_SIZE.x {
        for c_y in 0..MAP_SIZE.y {
            for x in 0..CHUNK_SIZE.x {
                for y in 0..CHUNK_SIZE.y {
                    let u = UVec2::new(x + c_x * CHUNK_SIZE.x, y + c_y * CHUNK_SIZE.y);
                    f_water(u, map);
                }
            }
        }
    }
}

fn generate_grass(c: UVec2, map: &mut BitMap) {
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let u = UVec2::new(x + c.x * CHUNK_SIZE.x, y + c.y * CHUNK_SIZE.y);

            if map.get_water(u) {
                continue;
            }
            if !map.in_bounds(u) {
                continue;
            }

            map.set_grass(u, 0);
        }
    }
}

fn generate_world(mut commands: Commands) {
    let mut map: BitMap = BitMap::default();

    generate_water(&mut map);
    filter_water(&mut map);

    for chunk_pos_x in 0..MAP_SIZE.x {
        for chunk_pos_y in 0..MAP_SIZE.y {
            let c = UVec2::new(chunk_pos_x, chunk_pos_y);
            generate_grass(c, &mut map);
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
