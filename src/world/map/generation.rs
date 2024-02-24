use std::collections::HashMap;

use rand::{thread_rng, Rng, SeedableRng};

use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d_seeded;

use crate::{GameRng, GameState};

use super::{CHUNK_SIZE, RENDERED_CHUNKS_RADIUS};

const SEED: f32 = 60.0;
const NOISE_ZOOM: f32 = 0.02;
const INVALID_TILE: u16 = 15 * 16;

const EMPTY_TYPE_INDEX: u8 = 0;
const WATER_TYPE_INDEX: u8 = 1;
const GRASS_TYPE_INDEX: u8 = 2;
const PATH_TYPE_INDEX: u8 = 3;

const BITMASK_TOP_RIGHT: u16 = 1 << 0;
const BITMASK_BOT_RIGHT: u16 = 1 << 1;
const BITMASK_BOT_LEFT: u16 = 1 << 2;
const BITMASK_TOP_LEFT: u16 = 1 << 3;

pub enum TileCollision {
    None,
    BotRect,
    LeftRect,
    TopRect,
    RightRect,
    BotLeftTri,
    TopLeftTri,
    TopRightTri,
    BotRightTri,
}

enum TileType {
    Grass,
    Path,
}

fn grid_to_index(x: u16, y: u16) -> u16 {
    x + y * 16
}

struct BitMasks {
    masks: HashMap<u16, Vec<u16>>,
}

impl BitMasks {
    fn grass() -> Self {
        Self {
            masks: HashMap::from([
                (
                    0,
                    vec![
                        grid_to_index(11, 13),
                        grid_to_index(12, 13),
                        grid_to_index(13, 13),
                        grid_to_index(12, 14),
                        grid_to_index(13, 14),
                    ],
                ),
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

    fn path() -> Self {
        Self {
            masks: HashMap::from([
                (0, vec![grid_to_index(0, 1)]),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(2, 1)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT,
                    vec![grid_to_index(1, 3)],
                ),
                (
                    BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(2, 3)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(2, 4)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_LEFT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(1, 4)],
                ),
                (
                    BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT,
                    vec![grid_to_index(2, 2), grid_to_index(4, 5)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(2, 0), grid_to_index(3, 5)],
                ),
                (
                    BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(1, 1), grid_to_index(2, 5)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_LEFT,
                    vec![grid_to_index(3, 1), grid_to_index(1, 5)],
                ),
                (BITMASK_BOT_RIGHT, vec![grid_to_index(1, 0)]),
                (BITMASK_BOT_LEFT, vec![grid_to_index(3, 0)]),
                (BITMASK_TOP_LEFT, vec![grid_to_index(3, 2)]),
                (BITMASK_TOP_RIGHT, vec![grid_to_index(1, 2)]),
                (
                    BITMASK_TOP_LEFT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(3, 4)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_RIGHT,
                    vec![grid_to_index(3, 3)],
                ),
            ]),
        }
    }
}

impl BitMasks {
    fn get_index(&self, mask: u16) -> u16 {
        let binding = vec![INVALID_TILE];
        let indices = self.masks.get(&mask).unwrap_or(&binding);
        let mut rng = thread_rng();
        let index = rng.gen_range(0..indices.len());
        indices[index]
    }
}

#[derive(Resource)]
pub struct BitMap {
    tile_q1: Vec<Vec<(u8, u16)>>,
    tile_q2: Vec<Vec<(u8, u16)>>,
    tile_q3: Vec<Vec<(u8, u16)>>,
    tile_q4: Vec<Vec<(u8, u16)>>,
}

impl Default for BitMap {
    fn default() -> Self {
        let length = CHUNK_SIZE as usize * RENDERED_CHUNKS_RADIUS as usize;
        Self {
            tile_q1: vec![vec![(EMPTY_TYPE_INDEX, INVALID_TILE); length]; length],
            tile_q2: vec![vec![(EMPTY_TYPE_INDEX, INVALID_TILE); length]; length],
            tile_q3: vec![vec![(EMPTY_TYPE_INDEX, INVALID_TILE); length]; length],
            tile_q4: vec![vec![(EMPTY_TYPE_INDEX, INVALID_TILE); length]; length],
        }
    }
}

impl BitMap {
    fn tileset_quadrant(&self, v: IVec2) -> u16 {
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

    fn tile_type(&mut self, v: IVec2) -> TileType {
        let has_water = self.collapse_water(v)
            | self.collapse_water(v + IVec2::new(0, 1))
            | self.collapse_water(v + IVec2::new(1, 1))
            | self.collapse_water(v + IVec2::new(1, 0));
        if has_water {
            TileType::Grass
        } else {
            TileType::Path
        }
    }

    fn get_tileset_raw(&mut self, v: IVec2) -> (u8, u16) {
        self.fit_tileset_size(v);

        let tileset = match self.tileset_quadrant(v) {
            1 => &mut self.tile_q1,
            2 => &mut self.tile_q2,
            3 => &mut self.tile_q3,
            4 => &mut self.tile_q4,
            _ => return (0, INVALID_TILE),
        };

        let x_index = v.x.abs() as usize;
        let y_index = v.y.abs() as usize;
        tileset[x_index][y_index]
    }

    fn get_tileset(&mut self, v: IVec2) -> u16 {
        self.get_tileset_raw(v).1
    }

    fn set_type_index(&mut self, v: IVec2, tile_type: u8) {
        self.fit_tileset_size(v);

        let tile = self.get_tileset_raw(v).1;
        let tile = (tile_type, tile);

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

    fn set_tileset(&mut self, v: IVec2, tile: u16) {
        self.fit_tileset_size(v);

        let tile_type = self.get_tileset_raw(v).0;
        let tile = (tile_type, tile);

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

    fn get_water_flag(&mut self, v: IVec2) -> bool {
        self.get_tileset_raw(v).0 == WATER_TYPE_INDEX
    }

    fn set_water_flag(&mut self, v: IVec2) {
        if self.get_tileset_raw(v).0 == EMPTY_TYPE_INDEX {
            self.set_type_index(v, WATER_TYPE_INDEX);
        }
    }

    fn get_path_flag(&mut self, v: IVec2) -> bool {
        self.get_tileset_raw(v).0 == PATH_TYPE_INDEX
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
                let mut addition = vec![
                    vec![(EMPTY_TYPE_INDEX, INVALID_TILE); tileset[0].len()];
                    CHUNK_SIZE as usize
                ];
                tileset.append(&mut addition);
            }
            if tileset[0].len() <= v.y.abs() as usize {
                fitting_size = false;
                for i in 0..tileset.len() {
                    let mut addition = vec![(EMPTY_TYPE_INDEX, INVALID_TILE); tileset[0].len()];
                    tileset[i].append(&mut addition);
                }
            }
        }
    }

    /// Determine if a given tile is water or grass.
    /// This will only set the water bit flag,
    /// not the actual tile index.
    fn collapse_water(&mut self, v: IVec2) -> bool {
        let tile_type = self.get_tileset_raw(v).0;
        if tile_type != EMPTY_TYPE_INDEX {
            return self.get_water_flag(v);
        }

        let w = Vec2::new(v.x as f32, v.y as f32);

        let noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, SEED);
        let secondary_noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, SEED + 1.0) * 1.0;
        let h = noise + secondary_noise;

        let is_water = h < 10.0;
        if is_water {
            self.set_water_flag(v);
        }
        is_water
    }

    /// Determine which tile to place. This will collapse
    /// the neigbhoring four tiles to see if they are grass or water.
    /// This is used for all tiles, both grass and water.
    fn collapse_tile(&mut self, v: IVec2) {
        let (mask, bitmask) = match self.tile_type(v) {
            TileType::Grass => (self.neigbhor_bitmask_grass(v), BitMasks::grass()),
            TileType::Path => (self.neigbhor_bitmask_path(v), BitMasks::path()),
        };

        let tile = bitmask.get_index(mask);
        self.set_tileset(v, tile);
    }

    /// Return the bitmask indicating which of the neigbhors are grass
    /// as 1 and which are water as 0.
    fn neigbhor_bitmask_grass(&mut self, v: IVec2) -> u16 {
        let mut mask = 0u16;

        mask |= BITMASK_BOT_LEFT * !self.collapse_water(v) as u16;
        mask |= BITMASK_TOP_LEFT * !self.collapse_water(v + IVec2::new(0, 1)) as u16;
        mask |= BITMASK_TOP_RIGHT * !self.collapse_water(v + IVec2::new(1, 1)) as u16;
        mask |= BITMASK_BOT_RIGHT * !self.collapse_water(v + IVec2::new(1, 0)) as u16;
        mask
    }

    fn neigbhor_bitmask_path(&mut self, v: IVec2) -> u16 {
        let mut mask = 0u16;

        mask |= BITMASK_BOT_LEFT * self.get_path_flag(v) as u16;
        mask |= BITMASK_TOP_LEFT * self.get_path_flag(v + IVec2::new(0, 1)) as u16;
        mask |= BITMASK_TOP_RIGHT * self.get_path_flag(v + IVec2::new(1, 1)) as u16;
        mask |= BITMASK_BOT_RIGHT * self.get_path_flag(v + IVec2::new(1, 0)) as u16;
        mask
    }

    /// Get the tile index for the given tile position.
    /// The tile index corresponds to the index in the tile atlas.
    /// It is not garuanteed to be a valid tile, i.e. it can be
    /// an invalid tile.
    pub fn get_tile_index(&mut self, v: IVec2) -> u16 {
        self.collapse_water(v);
        self.collapse_tile(v);
        self.get_tileset(v)
    }

    /// Determine if a given tile should have a collision and what type.
    pub fn get_tile_collision(&mut self, v: IVec2) -> TileCollision {
        let mask = self.neigbhor_bitmask_grass(v);
        if mask == !BITMASK_BOT_LEFT & BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT & !BITMASK_BOT_RIGHT {
            TileCollision::BotRect
        } else if mask
            == !BITMASK_BOT_LEFT & !BITMASK_TOP_LEFT & BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT
        {
            TileCollision::LeftRect
        } else if mask
            == BITMASK_BOT_LEFT & !BITMASK_TOP_LEFT & !BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT
        {
            TileCollision::TopRect
        } else if mask
            == BITMASK_BOT_LEFT | BITMASK_TOP_LEFT & !BITMASK_TOP_RIGHT & !BITMASK_BOT_RIGHT
        {
            TileCollision::RightRect
        } else if mask
            == !BITMASK_BOT_LEFT & !BITMASK_TOP_LEFT & BITMASK_TOP_RIGHT & !BITMASK_BOT_RIGHT
        {
            TileCollision::BotLeftTri
        } else if mask
            == !BITMASK_BOT_LEFT & !BITMASK_TOP_LEFT & !BITMASK_TOP_RIGHT & BITMASK_BOT_RIGHT
        {
            TileCollision::TopLeftTri
        } else if mask
            == BITMASK_BOT_LEFT & !BITMASK_TOP_LEFT & !BITMASK_TOP_RIGHT & !BITMASK_BOT_RIGHT
        {
            TileCollision::TopRightTri
        } else if mask
            == !BITMASK_BOT_LEFT & BITMASK_TOP_LEFT & !BITMASK_TOP_RIGHT & !BITMASK_BOT_RIGHT
        {
            TileCollision::BotRightTri
        } else {
            TileCollision::None
        }
    }
}

fn generate_bezier_curve_points(distance: f32, sample_size: usize) -> Vec<IVec2> {
    let mut rng = GameRng::seed_from_u64(SEED as u64);
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
    let distance = 200.0;
    let sample_size = 200;
    let min_radius: i32 = 1;
    let max_radius: i32 = min_radius + 3;
    let min_radius_grass: i32 = max_radius + 1;
    let max_radius_grass: i32 = min_radius_grass + 3;

    let points = generate_bezier_curve_points(distance, sample_size);

    for v in points {
        let w = Vec2::new(v.x as f32, v.y as f32);

        let noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, SEED);
        let secondary_noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, SEED + 1.0) * 1.0;
        let radius = (min_radius as f32
            + 0.25 * (noise + secondary_noise + 2.0) * (max_radius - min_radius) as f32)
            as i32;
        let sqrt_radius = radius.pow(2);
        let radius_grass = (min_radius_grass as f32
            + 0.25 * (noise + secondary_noise + 2.0) * (max_radius_grass - min_radius_grass) as f32)
            as i32;
        let sqrt_radius_grass = radius_grass.pow(2);

        for x in -radius_grass..=radius_grass {
            for y in -radius_grass..=radius_grass {
                let offset = IVec2::new(x, y);
                let dis = offset.length_squared();
                if dis < sqrt_radius {
                    bitmap.set_type_index(v + offset, PATH_TYPE_INDEX);
                } else if dis < sqrt_radius_grass
                    && bitmap.get_tileset_raw(v + offset).0 != PATH_TYPE_INDEX
                {
                    bitmap.set_type_index(v + offset, GRASS_TYPE_INDEX);
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
