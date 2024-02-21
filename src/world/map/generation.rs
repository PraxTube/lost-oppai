use rand::{thread_rng, Rng};

use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d_seeded;

use super::{CHUNK_SIZE, RENDERED_CHUNKS_RADIUS};

const SEED: f32 = 60.0;
const NOISE_ZOOM: f32 = 0.04;
const WATER: u8 = 0;
const PRIMITIVE_GRASS: u8 = 1;
const INVALID_TILE: u8 = 15 * 16;
const EMPTY: u8 = 15 * 16 + 1;

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
                    BITMASK_TOP_LEFT | BITMASK_RIGHT,
                    vec![grid_to_index(6, 6)],
                ),
                (
                    BITMASK_TOP | BITMASK_RIGHT | BITMASK_BOT_RIGHT | BITMASK_BOT,
                    BITMASK_TOP_RIGHT | BITMASK_LEFT,
                    vec![grid_to_index(6, 7)],
                ),
                (
                    BITMASK_TOP | BITMASK_LEFT | BITMASK_TOP_LEFT | BITMASK_RIGHT,
                    BITMASK_TOP_RIGHT | BITMASK_BOT,
                    vec![grid_to_index(7, 6)],
                ),
                (
                    BITMASK_RIGHT | BITMASK_LEFT | BITMASK_BOT_LEFT | BITMASK_LEFT,
                    BITMASK_BOT_RIGHT | BITMASK_TOP,
                    vec![grid_to_index(7, 7)],
                ),
                (
                    BITMASK_TOP | BITMASK_LEFT | BITMASK_RIGHT | BITMASK_TOP_RIGHT,
                    BITMASK_TOP_LEFT | BITMASK_BOT,
                    vec![grid_to_index(8, 6)],
                ),
                (
                    BITMASK_LEFT | BITMASK_BOT | BITMASK_BOT_RIGHT | BITMASK_RIGHT,
                    BITMASK_BOT_LEFT | BITMASK_TOP,
                    vec![grid_to_index(8, 7)],
                ),
                (
                    BITMASK_TOP | BITMASK_LEFT | BITMASK_BOT | BITMASK_TOP_LEFT,
                    BITMASK_BOT_LEFT | BITMASK_RIGHT,
                    vec![grid_to_index(9, 6)],
                ),
                (
                    BITMASK_BOT | BITMASK_TOP | BITMASK_TOP_RIGHT | BITMASK_RIGHT,
                    BITMASK_BOT_RIGHT | BITMASK_LEFT,
                    vec![grid_to_index(9, 7)],
                ),
                (
                    BITMASK_TOP | BITMASK_BOT,
                    BITMASK_LEFT | BITMASK_RIGHT,
                    vec![grid_to_index(7, 3)],
                ),
                (
                    BITMASK_LEFT | BITMASK_RIGHT,
                    BITMASK_TOP | BITMASK_BOT,
                    vec![grid_to_index(8, 3)],
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
        EMPTY
    }
}

#[derive(Resource)]
pub struct BitMap {
    tile_q1: Vec<Vec<u8>>,
    tile_q2: Vec<Vec<u8>>,
    tile_q3: Vec<Vec<u8>>,
    tile_q4: Vec<Vec<u8>>,
}

impl Default for BitMap {
    fn default() -> Self {
        let length = CHUNK_SIZE as usize * RENDERED_CHUNKS_RADIUS as usize;
        Self {
            tile_q1: vec![vec![INVALID_TILE; length]; length],
            tile_q2: vec![vec![INVALID_TILE; length]; length],
            tile_q3: vec![vec![INVALID_TILE; length]; length],
            tile_q4: vec![vec![INVALID_TILE; length]; length],
        }
    }
}

impl BitMap {
    fn increase_tileset(&mut self, v: IVec2) {
        let tileset = match self.tileset_quadrant(v) {
            1 => &mut self.tile_q1,
            2 => &mut self.tile_q2,
            3 => &mut self.tile_q3,
            4 => &mut self.tile_q4,
            _ => return,
        };

        let mut fitting_size = false;

        while !fitting_size {
            fitting_size = true;
            if tileset.len() <= v.x.abs() as usize {
                fitting_size = false;
                let mut addition = vec![vec![INVALID_TILE; tileset[0].len()]; CHUNK_SIZE as usize];
                tileset.append(&mut addition);
            }
            if tileset[0].len() <= v.y.abs() as usize {
                fitting_size = false;
                for i in 0..tileset.len() {
                    let mut addition = vec![INVALID_TILE; tileset[0].len()];
                    tileset[i].append(&mut addition);
                }
            }
        }
    }

    fn tileset_quadrant(&self, v: IVec2) -> u8 {
        if v.x >= 0 && v.y >= 0 {
            1
        } else if v.x < 0 && v.y >= 0 {
            2
        } else if v.x < 0 && v.y < 0 {
            3
        } else if v.x >= 0 && v.y < 0 {
            4
        } else {
            0
        }
    }

    fn collapse_water(&mut self, v: IVec2) -> bool {
        self.increase_tileset(v);
        let w = Vec2::new(v.x as f32, v.y as f32);

        let noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, SEED);
        let secondary_noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, SEED + 1.0) * 1.0;
        let h = noise + secondary_noise;

        let tile = if h < 0.0 { WATER } else { PRIMITIVE_GRASS };

        let tileset = match self.tileset_quadrant(v) {
            1 => &mut self.tile_q1,
            2 => &mut self.tile_q2,
            3 => &mut self.tile_q3,
            4 => &mut self.tile_q4,
            _ => return false,
        };

        let x_index = v.x.abs() as usize;
        let y_index = v.y.abs() as usize;
        tileset[x_index][y_index] = tile;
        tile == WATER
    }

    fn collapse_grass(&mut self, v: IVec2) {
        self.increase_tileset(v);
        let x_index = v.x.abs() as usize;
        let y_index = v.y.abs() as usize;

        let determine_grass_tile = if let Some(tileset) = match self.tileset_quadrant(v) {
            1 => Some(&mut self.tile_q1),
            2 => Some(&mut self.tile_q2),
            3 => Some(&mut self.tile_q3),
            4 => Some(&mut self.tile_q4),
            _ => None,
        } {
            tileset[x_index][y_index] == PRIMITIVE_GRASS
        } else {
            false
        };

        if determine_grass_tile {
            self.determine_grass_tile(v);
        }
    }

    fn get_tile_index(&mut self, v: IVec2) -> u8 {
        self.increase_tileset(v);
        self.collapse_water(v);
        self.collapse_grass(v);

        let x_index = v.x.abs() as usize;
        let y_index = v.y.abs() as usize;

        let tileset = match self.tileset_quadrant(v) {
            1 => &mut self.tile_q1,
            2 => &mut self.tile_q2,
            3 => &mut self.tile_q3,
            4 => &mut self.tile_q4,
            _ => return INVALID_TILE,
        };

        tileset[x_index][y_index]
    }

    fn neigbhor_bitmask(&mut self, v: IVec2) -> u8 {
        let mut mask = 0u8;

        mask |= BITMASK_TOP * !self.is_water(v + IVec2::new(0, 1)) as u8;
        mask |= BITMASK_TOP_RIGHT * !self.is_water(v + IVec2::new(1, 1)) as u8;
        mask |= BITMASK_RIGHT * !self.is_water(v + IVec2::new(1, 0)) as u8;
        mask |= BITMASK_BOT_RIGHT * !self.is_water(v + IVec2::new(1, -1)) as u8;
        mask |= BITMASK_BOT * !self.is_water(v + IVec2::new(0, -1)) as u8;
        mask |= BITMASK_BOT_LEFT * !self.is_water(v + IVec2::new(-1, -1)) as u8;
        mask |= BITMASK_LEFT * !self.is_water(v + IVec2::new(-1, 0)) as u8;
        mask |= BITMASK_TOP_LEFT * !self.is_water(v + IVec2::new(-1, 1)) as u8;
        mask
    }

    fn determine_grass_tile(&mut self, v: IVec2) {
        let mask = self.neigbhor_bitmask(v);
        let bitmask = GrassBitMask::default();
        let tile = bitmask.get_index(mask);

        let tileset = match self.tileset_quadrant(v) {
            1 => &mut self.tile_q1,
            2 => &mut self.tile_q2,
            3 => &mut self.tile_q3,
            4 => &mut self.tile_q4,
            _ => return,
        };

        let x_index = v.x.abs() as usize;
        let y_index = v.y.abs() as usize;
        tileset[x_index][y_index] = tile;
    }

    fn is_water(&mut self, v: IVec2) -> bool {
        self.collapse_water(v)
    }

    pub fn get(&mut self, v: IVec2) -> u8 {
        self.get_tile_index(v)
    }

    pub fn is_collision(&mut self, v: IVec2) -> bool {
        if !self.is_water(v) {
            return false;
        }
        self.neigbhor_bitmask(v) != 0
    }
}

pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BitMap>();
    }
}
