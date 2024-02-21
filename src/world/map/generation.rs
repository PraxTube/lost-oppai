use std::collections::HashMap;

use rand::{thread_rng, Rng};

use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d_seeded;

use crate::GameState;

use super::{CHUNK_SIZE, RENDERED_CHUNKS_RADIUS};

const SEED: f32 = 60.0;
const NOISE_ZOOM: f32 = 0.02;
const WATER: u8 = 0;
const PRIMITIVE_GRASS: u8 = 1;
const INVALID_TILE: u8 = 15 * 16;
const EMPTY: u8 = 15 * 16 + 1;

const BITMASK_TOP_RIGHT: u8 = 1 << 1;
const BITMASK_BOT_RIGHT: u8 = 1 << 3;
const BITMASK_BOT_LEFT: u8 = 1 << 5;
const BITMASK_TOP_LEFT: u8 = 1 << 7;

struct GrassBitMask {
    masks: HashMap<u8, Vec<u8>>,
}

impl Default for GrassBitMask {
    fn default() -> Self {
        fn grid_to_index(x: u8, y: u8) -> u8 {
            x + y * 16
        }

        Self {
            masks: HashMap::from([
                (0, vec![grid_to_index(0, 0)]),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(0, 1)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT,
                    vec![grid_to_index(5, 0)],
                ),
                (
                    BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(8, 0)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(8, 2)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_LEFT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(5, 2)],
                ),
                (
                    BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT,
                    vec![grid_to_index(6, 0), grid_to_index(7, 0)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(6, 2), grid_to_index(7, 2)],
                ),
                (
                    BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(7, 1), grid_to_index(8, 1)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_LEFT,
                    vec![grid_to_index(5, 1), grid_to_index(6, 1)],
                ),
                (BITMASK_BOT_RIGHT, vec![grid_to_index(5, 3)]),
                (BITMASK_BOT_LEFT, vec![grid_to_index(6, 3)]),
                (BITMASK_TOP_LEFT, vec![grid_to_index(6, 4)]),
                (BITMASK_TOP_RIGHT, vec![grid_to_index(5, 4)]),
                (
                    BITMASK_TOP_LEFT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(7, 3)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_RIGHT,
                    vec![grid_to_index(7, 4)],
                ),
            ]),
        }
    }
}

impl GrassBitMask {
    fn get_index(&self, mask: u8) -> u8 {
        let binding = vec![EMPTY];
        let indices = self.masks.get(&mask).unwrap_or(&binding);
        let mut rng = thread_rng();
        let index = rng.gen_range(0..indices.len());
        indices[index]
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

    fn get_tileset(&mut self, v: IVec2) -> u8 {
        self.fit_tileset_size(v);

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

    fn set_tileset(&mut self, v: IVec2, tile: u8) {
        self.fit_tileset_size(v);

        let x_index = v.x.abs() as usize;
        let y_index = v.y.abs() as usize;

        let tileset = match self.tileset_quadrant(v) {
            1 => &mut self.tile_q1,
            2 => &mut self.tile_q2,
            3 => &mut self.tile_q3,
            4 => &mut self.tile_q4,
            _ => return,
        };

        tileset[x_index][y_index] = tile;
    }

    fn fit_tileset_size(&mut self, v: IVec2) {
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

    fn collapse_water(&mut self, v: IVec2) -> bool {
        let tile = self.get_tileset(v);
        if tile != INVALID_TILE {
            return tile == WATER;
        }

        let w = Vec2::new(v.x as f32, v.y as f32);

        let noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, SEED);
        let secondary_noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, SEED + 1.0) * 1.0;
        let h = noise + secondary_noise;

        // let tile = if h < 0.0 { WATER } else { PRIMITIVE_GRASS };
        let tile = if h < 10.0 { WATER } else { PRIMITIVE_GRASS };

        self.set_tileset(v, tile);
        tile == WATER
    }

    fn collapse_grass(&mut self, v: IVec2) {
        self.determine_grass_tile(v);
        if self.get_tileset(v) == PRIMITIVE_GRASS {}
    }

    fn is_water(&mut self, v: IVec2) -> bool {
        self.collapse_water(v)
    }

    fn get_tile_index(&mut self, v: IVec2) -> u8 {
        self.collapse_water(v);
        self.collapse_grass(v);

        self.get_tileset(v)
    }

    /// Return the bitmask indicating which of the neigbhors are grass
    /// as 1 and which are water as 0.
    fn neigbhor_bitmask(&mut self, v: IVec2) -> u8 {
        let mut mask = 0u8;

        mask |= BITMASK_BOT_LEFT * !self.is_water(v) as u8;
        mask |= BITMASK_TOP_LEFT * !self.is_water(v + IVec2::new(0, 1)) as u8;
        mask |= BITMASK_TOP_RIGHT * !self.is_water(v + IVec2::new(1, 1)) as u8;
        mask |= BITMASK_BOT_RIGHT * !self.is_water(v + IVec2::new(1, 0)) as u8;
        mask
    }

    fn determine_grass_tile(&mut self, v: IVec2) {
        let mask = self.neigbhor_bitmask(v);
        let bitmask = GrassBitMask::default();
        let tile = bitmask.get_index(mask);
        self.set_tileset(v, tile);
    }

    /// Determine if a given tile should have a collision box.
    /// If the tile is a water tile that has at least one neigbhoring
    /// grass tile, then it needs to have a collision.
    pub fn is_collision(&mut self, v: IVec2) -> bool {
        if !self.is_water(v) {
            return false;
        }
        self.neigbhor_bitmask(v) != 0
    }

    /// Get the tile index for the given tile position.
    /// The tile index corresponds to the index in the tile atlas.
    /// It is not garuanteed to be a valid tile, i.e. it can be
    /// an invalid tile.
    pub fn get(&mut self, v: IVec2) -> u8 {
        self.get_tile_index(v)
    }
}

fn generate_bezier_curve_points(distance: f32, sample_size: usize) -> Vec<IVec2> {
    let mut rng = thread_rng();
    let x = rng.gen_range(-distance..distance);
    let y_sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
    let y = y_sign * (distance.powi(2) - x.powi(2)).sqrt();
    let p1 = Vec2::ZERO;
    let p2 = Vec2::new(x, y);

    let min_p = Vec2::new(p1.x.min(p2.x), p1.y.min(p2.y));
    let max_p = Vec2::new(p1.x.max(p2.x), p1.y.max(p2.y));
    let c1 = Vec2::new(
        rng.gen_range(min_p.x..max_p.x),
        rng.gen_range(min_p.y..max_p.y),
    );
    let c2 = Vec2::new(
        rng.gen_range(min_p.x..max_p.x),
        rng.gen_range(min_p.y..max_p.y),
    );

    fn curve(p1: Vec2, p2: Vec2, c1: Vec2, c2: Vec2, t: f32) -> Vec2 {
        (1.0 - t).powi(3) * p1
            + 3.0 * t * (1.0 - t).powi(2) * c1
            + 3.0 * t.powi(2) * (1.0 - t) * c2
            + t.powi(3) * p2
    }

    let mut points = Vec::new();
    for i in 0..sample_size {
        let t = i as f32 / sample_size as f32;
        let pos = curve(p1, p2, c1, c2, t);
        let dis_pos = IVec2::new(pos.x as i32, pos.y as i32);
        points.push(dis_pos);
    }
    points
}

fn generate_path(mut bitmap: ResMut<BitMap>) {
    let distance = 100.0;
    let sample_size = 100;
    let max_radius: i32 = 10;
    let min_radius: i32 = 2;

    let points = generate_bezier_curve_points(distance, sample_size);

    for v in points {
        let w = Vec2::new(v.x as f32, v.y as f32);

        let noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, SEED);
        let secondary_noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, SEED + 1.0) * 1.0;
        let radius = (min_radius as f32
            + 0.25 * (noise + secondary_noise + 2.0) * (max_radius - min_radius) as f32)
            as i32;
        let sqrt_radius = radius.pow(2);

        for x in -radius..=radius {
            for y in -radius..=radius {
                let offset = IVec2::new(x, y);
                if offset.length_squared() < sqrt_radius {
                    bitmap.set_tileset(v + offset, PRIMITIVE_GRASS);
                }
            }
        }
    }
}

pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BitMap>()
            .add_systems(OnExit(GameState::AssetLoading), (generate_path,));
    }
}
