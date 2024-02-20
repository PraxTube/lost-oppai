use rand::{thread_rng, Rng};

use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d_seeded;

const SEED: f32 = 60.0;
const NOISE_ZOOM: f32 = 0.04;
const WATER: u8 = 0;

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
        15 * 16
    }
}

#[derive(Default, Resource)]
pub struct BitMap;

impl BitMap {
    fn neigbhor_bitmask(&self, v: IVec2) -> u8 {
        let mut mask = 0u8;

        mask |= BITMASK_TOP * !self.get_water(v + IVec2::new(0, 1)) as u8;
        mask |= BITMASK_TOP_RIGHT * !self.get_water(v + IVec2::new(1, 1)) as u8;
        mask |= BITMASK_RIGHT * !self.get_water(v + IVec2::new(1, 0)) as u8;
        mask |= BITMASK_BOT_RIGHT * !self.get_water(v + IVec2::new(1, -1)) as u8;
        mask |= BITMASK_BOT * !self.get_water(v + IVec2::new(0, -1)) as u8;
        mask |= BITMASK_BOT_LEFT * !self.get_water(v + IVec2::new(-1, -1)) as u8;
        mask |= BITMASK_LEFT * !self.get_water(v + IVec2::new(-1, 0)) as u8;
        mask |= BITMASK_TOP_LEFT * !self.get_water(v + IVec2::new(-1, 1)) as u8;
        mask
    }

    fn determine_grass_tile(&self, v: IVec2) -> u8 {
        let mask = self.neigbhor_bitmask(v);
        let bitmask = GrassBitMask::default();
        bitmask.get_index(mask)
    }

    fn get_water(&self, v: IVec2) -> bool {
        let w = Vec2::new(v.x as f32, v.y as f32);

        let noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, SEED);
        let secondary_noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, SEED + 1.0) * 1.0;
        let h = noise + secondary_noise;

        h < 0.0
    }

    pub fn get(&self, v: IVec2) -> u8 {
        if self.get_water(v) {
            return WATER;
        }

        self.determine_grass_tile(v)
    }

    pub fn is_collision(&self, v: IVec2) -> bool {
        if !self.get_water(v) {
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
