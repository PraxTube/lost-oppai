use rand::{thread_rng, Rng};

use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d_seeded;

use super::CHUNK_SIZE;

const SEED: f32 = 60.0;
const NOISE_ZOOM: f32 = 0.04;
const MAP_SIZE: UVec2 = UVec2::new(10, 10);
const WATER: u8 = 0;
const EMPTY_INDEX: u8 = WATER;

const BITMASK_TOP: u8 = 1 << 0;
const BITMASK_TOP_RIGHT: u8 = 1 << 1;
const BITMASK_RIGHT: u8 = 1 << 2;
const BITMASK_BOT_RIGHT: u8 = 1 << 3;
const BITMASK_BOT: u8 = 1 << 4;
const BITMASK_BOT_LEFT: u8 = 1 << 5;
const BITMASK_LEFT: u8 = 1 << 6;
const BITMASK_TOP_LEFT: u8 = 1 << 7;

struct GrassBitMask {
    masks: Vec<(u8, u8, Vec<u8>)>,
}

impl Default for GrassBitMask {
    fn default() -> Self {
        fn grid_to_index(x: u8, y: u8) -> u8 {
            x + y * 16
        }

        Self {
            masks: vec![
                (
                    BITMASK_RIGHT | BITMASK_BOT_RIGHT | BITMASK_BOT,
                    BITMASK_TOP | BITMASK_LEFT,
                    vec![grid_to_index(6, 0)],
                ),
                (
                    BITMASK_LEFT | BITMASK_BOT_LEFT | BITMASK_BOT,
                    BITMASK_TOP | BITMASK_RIGHT,
                    vec![grid_to_index(9, 0)],
                ),
                (
                    BITMASK_TOP | BITMASK_TOP_RIGHT | BITMASK_RIGHT,
                    BITMASK_BOT | BITMASK_LEFT,
                    vec![grid_to_index(6, 14)],
                ),
                (
                    BITMASK_TOP | BITMASK_TOP_LEFT | BITMASK_LEFT,
                    BITMASK_BOT | BITMASK_RIGHT,
                    vec![grid_to_index(9, 14)],
                ),
                (
                    BITMASK_TOP
                        | BITMASK_TOP_RIGHT
                        | BITMASK_RIGHT
                        | BITMASK_BOT_RIGHT
                        | BITMASK_BOT,
                    BITMASK_LEFT,
                    vec![grid_to_index(5, 2), grid_to_index(6, 2)],
                ),
                (
                    BITMASK_LEFT
                        | BITMASK_BOT_LEFT
                        | BITMASK_BOT
                        | BITMASK_BOT_RIGHT
                        | BITMASK_RIGHT,
                    BITMASK_TOP,
                    vec![grid_to_index(7, 0), grid_to_index(8, 0)],
                ),
                (
                    BITMASK_LEFT
                        | BITMASK_TOP_LEFT
                        | BITMASK_TOP
                        | BITMASK_TOP_RIGHT
                        | BITMASK_RIGHT,
                    BITMASK_BOT,
                    vec![grid_to_index(7, 14), grid_to_index(8, 14)],
                ),
                (
                    BITMASK_TOP | BITMASK_TOP_LEFT | BITMASK_LEFT | BITMASK_BOT_LEFT | BITMASK_BOT,
                    BITMASK_RIGHT,
                    vec![grid_to_index(9, 2), grid_to_index(10, 2)],
                ),
                (
                    !BITMASK_TOP_LEFT,
                    BITMASK_TOP_LEFT,
                    vec![grid_to_index(6, 1)],
                ),
                (
                    !BITMASK_TOP_RIGHT,
                    BITMASK_TOP_RIGHT,
                    vec![grid_to_index(9, 1)],
                ),
                (
                    !BITMASK_BOT_RIGHT,
                    BITMASK_BOT_RIGHT,
                    vec![grid_to_index(9, 3)],
                ),
                (
                    !BITMASK_BOT_LEFT,
                    BITMASK_BOT_LEFT,
                    vec![grid_to_index(6, 3)],
                ),
                (
                    u8::MAX,
                    0,
                    vec![
                        grid_to_index(0, 1),
                        grid_to_index(0, 2),
                        grid_to_index(0, 3),
                        grid_to_index(0, 4),
                        grid_to_index(0, 5),
                    ],
                ),
                (
                    BITMASK_BOT,
                    BITMASK_LEFT
                        | BITMASK_TOP_LEFT
                        | BITMASK_TOP
                        | BITMASK_TOP_RIGHT
                        | BITMASK_RIGHT,
                    vec![grid_to_index(6, 5)],
                ),
                (
                    BITMASK_LEFT,
                    BITMASK_TOP
                        | BITMASK_TOP_RIGHT
                        | BITMASK_RIGHT
                        | BITMASK_BOT_RIGHT
                        | BITMASK_BOT,
                    vec![grid_to_index(7, 5)],
                ),
                (
                    BITMASK_RIGHT,
                    BITMASK_TOP | BITMASK_TOP_LEFT | BITMASK_LEFT | BITMASK_BOT_LEFT | BITMASK_BOT,
                    vec![grid_to_index(8, 5)],
                ),
                (
                    BITMASK_TOP,
                    BITMASK_LEFT
                        | BITMASK_BOT_LEFT
                        | BITMASK_RIGHT
                        | BITMASK_BOT_RIGHT
                        | BITMASK_BOT,
                    vec![grid_to_index(9, 5)],
                ),
                (
                    !BITMASK_TOP_LEFT & !BITMASK_TOP_RIGHT,
                    BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT,
                    vec![grid_to_index(7, 1)],
                ),
                (
                    !BITMASK_BOT_LEFT & !BITMASK_BOT_RIGHT,
                    BITMASK_BOT_LEFT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(8, 1)],
                ),
                (
                    !BITMASK_BOT_LEFT & !BITMASK_TOP_LEFT,
                    BITMASK_BOT_LEFT | BITMASK_TOP_LEFT,
                    vec![grid_to_index(7, 2)],
                ),
                (
                    !BITMASK_BOT_RIGHT & !BITMASK_TOP_RIGHT,
                    BITMASK_BOT_RIGHT | BITMASK_TOP_RIGHT,
                    vec![grid_to_index(8, 2)],
                ),
                (
                    BITMASK_TOP | BITMASK_LEFT | BITMASK_BOT_LEFT | BITMASK_BOT,
                    BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT | BITMASK_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(6, 6)],
                ),
                (
                    BITMASK_TOP | BITMASK_RIGHT | BITMASK_BOT_RIGHT | BITMASK_BOT,
                    BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT | BITMASK_LEFT | BITMASK_BOT_LEFT,
                    vec![grid_to_index(6, 7)],
                ),
                (
                    BITMASK_TOP | BITMASK_LEFT | BITMASK_TOP_LEFT | BITMASK_RIGHT,
                    BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT | BITMASK_BOT | BITMASK_BOT_LEFT,
                    vec![grid_to_index(7, 6)],
                ),
                (
                    BITMASK_RIGHT | BITMASK_LEFT | BITMASK_BOT_LEFT | BITMASK_LEFT,
                    BITMASK_BOT_RIGHT | BITMASK_TOP_LEFT | BITMASK_TOP | BITMASK_TOP_RIGHT,
                    vec![grid_to_index(7, 7)],
                ),
                (
                    BITMASK_TOP | BITMASK_LEFT | BITMASK_RIGHT | BITMASK_TOP_RIGHT,
                    BITMASK_TOP_LEFT | BITMASK_BOT_RIGHT | BITMASK_BOT | BITMASK_BOT_LEFT,
                    vec![grid_to_index(8, 6)],
                ),
                (
                    BITMASK_LEFT | BITMASK_BOT | BITMASK_BOT_RIGHT | BITMASK_RIGHT,
                    BITMASK_BOT_LEFT | BITMASK_TOP_RIGHT | BITMASK_TOP | BITMASK_TOP_RIGHT,
                    vec![grid_to_index(8, 7)],
                ),
                (
                    BITMASK_TOP | BITMASK_LEFT | BITMASK_BOT | BITMASK_TOP_LEFT,
                    BITMASK_BOT_LEFT | BITMASK_TOP_RIGHT | BITMASK_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(9, 6)],
                ),
                (
                    BITMASK_BOT | BITMASK_TOP | BITMASK_TOP_RIGHT | BITMASK_RIGHT,
                    BITMASK_BOT_RIGHT | BITMASK_BOT_LEFT | BITMASK_LEFT | BITMASK_TOP_LEFT,
                    vec![grid_to_index(9, 7)],
                ),
                (
                    BITMASK_TOP | BITMASK_BOT,
                    BITMASK_LEFT | BITMASK_RIGHT,
                    vec![grid_to_index(6, 9)],
                ),
                (
                    BITMASK_LEFT | BITMASK_RIGHT,
                    BITMASK_TOP | BITMASK_BOT,
                    vec![grid_to_index(7, 9)],
                ),
            ],
        }
    }
}

impl GrassBitMask {
    fn get_index(&self, mask: u8) -> u8 {
        for (tile_mask, not_tile_mask, indices) in &self.masks {
            if mask & tile_mask != *tile_mask {
                continue;
            }
            if !mask & not_tile_mask != *not_tile_mask {
                continue;
            }

            let mut rng = thread_rng();
            let index = rng.gen_range(0..indices.len());
            return indices[index];
        }

        warn!("Tile that doesn't map to any grass tile, mask: {}", mask);
        15 * 16
    }
}

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
    /// All the mask matrices should always have the same size,
    /// so use water as a represent of all of them
    fn in_bounds(&self, u: UVec2) -> bool {
        let max_x = self.water.len() as u32 - 1;
        let max_y = self.water[0].len() as u32 - 1;

        u.x > 0 && u.y > 0 && u.x < max_x && u.y < max_y
    }

    fn neigbhor_bitmask(&self, u: UVec2) -> u8 {
        if !self.in_bounds(u) {
            return 0;
        }

        let mut mask = 0u8;

        mask |= BITMASK_TOP * !self.get_water(u + UVec2::new(0, 1)) as u8;
        mask |= BITMASK_TOP_RIGHT * !self.get_water(u + UVec2::new(1, 1)) as u8;
        mask |= BITMASK_RIGHT * !self.get_water(u + UVec2::new(1, 0)) as u8;
        mask |= BITMASK_BOT_RIGHT * !self.get_water(u + UVec2::new(1, 0) - UVec2::new(0, 1)) as u8;
        mask |= BITMASK_BOT * !self.get_water(u - UVec2::new(0, 1)) as u8;
        mask |= BITMASK_BOT_LEFT * !self.get_water(u - UVec2::new(1, 1)) as u8;
        mask |= BITMASK_LEFT * !self.get_water(u - UVec2::new(1, 0)) as u8;
        mask |= BITMASK_TOP_LEFT * !self.get_water(u + UVec2::new(0, 1) - UVec2::new(1, 0)) as u8;
        mask
    }

    fn determine_grass_tile(&self, u: UVec2) -> u8 {
        let mask = self.neigbhor_bitmask(u);
        let bitmask = GrassBitMask::default();
        bitmask.get_index(mask)
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

    pub fn v_to_u(&self, v: IVec2) -> UVec2 {
        let u = v + IVec2::new(
            (CHUNK_SIZE.x * MAP_SIZE.x / 2) as i32,
            (CHUNK_SIZE.y * MAP_SIZE.y / 2) as i32,
        );
        UVec2::new(u.x as u32, u.y as u32)
    }

    pub fn get(&self, v: IVec2) -> u8 {
        let u = self.v_to_u(v);
        if u.x as usize >= self.grass.len() || u.y as usize >= self.grass[0].len() {
            return EMPTY_INDEX;
        }
        self.grass[u.x as usize][u.y as usize]
    }

    pub fn is_collision(&self, v: IVec2) -> bool {
        let u = self.v_to_u(v);
        if !self.get_water(u) {
            return false;
        }
        self.neigbhor_bitmask(u) != 0
    }
}

fn generate_water(map: &mut BitMap) {
    fn g_water(u: UVec2, map: &mut BitMap) {
        if !map.in_bounds(u) {
            return;
        }

        let v = Vec2::new(u.x as f32, u.y as f32);

        let noise = simplex_noise_2d_seeded(v * NOISE_ZOOM, SEED);
        let secondary_noise = simplex_noise_2d_seeded(v * NOISE_ZOOM, SEED + 1.0) * 1.0;
        let h = noise + secondary_noise;

        let b = h < 0.0;
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

            map.set_grass(u, map.determine_grass_tile(u));
        }
    }
}

fn generate_world(mut commands: Commands) {
    let mut map: BitMap = BitMap::default();

    generate_water(&mut map);
    // filter_water(&mut map);

    for chunk_pos_x in 0..MAP_SIZE.x {
        for chunk_pos_y in 0..MAP_SIZE.y {
            let c = UVec2::new(chunk_pos_x, chunk_pos_y);
            generate_grass(c, &mut map);
        }
    }

    commands.insert_resource(map);
}

pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_world);
    }
}
