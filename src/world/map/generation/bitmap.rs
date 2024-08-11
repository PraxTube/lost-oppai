use bevy::prelude::*;
use bevy::utils::HashSet;
use chrono::{Timelike, Utc};
use noisy_bevy::simplex_noise_2d_seeded;

use crate::world::map::TILE_SIZE;

use super::bitmask::{BitMasks, GRASS_FLOWER_SUPER_POSITION};
use super::{
    TileCollision, TileType, BITMASK_BOT_LEFT, BITMASK_BOT_RIGHT, BITMASK_TOP_LEFT,
    BITMASK_TOP_RIGHT, CHUNK_SIZE, EMPTY_TYPE_MASK, FLOWER_HEIGHT_LEVEL, FLOWER_NOISE_ZOOM,
    GRASS_TYPE_MASK, INVALID_TILE, NOISE_ZOOM, PATH_TYPE_MASK, RENDERED_CHUNKS_RADIUS,
    WATER_HEIGH_LEVEL, WATER_SPARKLE_HEIGHT_LEVEL_MAX, WATER_SPARKLE_HEIGHT_LEVEL_MIN,
    WATER_SPARKLE_NOISE_ZOOM, WATER_SPARKLE_TYPE_MASK, WATER_TYPE_MASK,
};

#[derive(Resource)]
pub struct BitMap {
    seed: f32,
    vertices: Vec<Vec2>,
    edges: HashSet<(usize, usize)>,

    grass_mask: BitMasks,
    path_mask: BitMasks,
    flower_mask: BitMasks,
    water_sparkle_mask: BitMasks,

    tile_q1: Vec<Vec<(u8, u16)>>,
    tile_q2: Vec<Vec<(u8, u16)>>,
    tile_q3: Vec<Vec<(u8, u16)>>,
    tile_q4: Vec<Vec<(u8, u16)>>,
}

impl Default for BitMap {
    fn default() -> Self {
        let length = CHUNK_SIZE as usize * RENDERED_CHUNKS_RADIUS as usize;
        Self {
            seed: Utc::now().nanosecond() as f32,
            vertices: Vec::new(),
            edges: HashSet::new(),

            grass_mask: BitMasks::grass(),
            path_mask: BitMasks::path(),
            flower_mask: BitMasks::flower(),
            water_sparkle_mask: BitMasks::water_sparkle(),

            tile_q1: vec![vec![(EMPTY_TYPE_MASK, INVALID_TILE); length]; length],
            tile_q2: vec![vec![(EMPTY_TYPE_MASK, INVALID_TILE); length]; length],
            tile_q3: vec![vec![(EMPTY_TYPE_MASK, INVALID_TILE); length]; length],
            tile_q4: vec![vec![(EMPTY_TYPE_MASK, INVALID_TILE); length]; length],
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
            TileType::GrassWater
        } else {
            TileType::PathOrGrass
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

        let x_index = v.x.unsigned_abs() as usize;
        let y_index = v.y.unsigned_abs() as usize;
        tileset[x_index][y_index]
    }

    fn get_tileset(&mut self, v: IVec2) -> u16 {
        self.get_tileset_raw(v).1
    }

    pub fn set_type_index(&mut self, v: IVec2, tile_type: u8) {
        if tile_type == EMPTY_TYPE_MASK {
            error!(
                "Setting tile type to empty tile index, should never happen, {}",
                v
            );
        }

        let t = self.get_tileset_raw(v);
        let tile_type = if tile_type == WATER_SPARKLE_TYPE_MASK {
            if t.0 & WATER_TYPE_MASK != WATER_TYPE_MASK {
                warn!(
                    "Setting water sparkle mask on a tile that is not water! mask: {}",
                    t.0
                );
            }
            (t.0 & WATER_TYPE_MASK) | tile_type
        } else {
            tile_type
        };

        let tile = (tile_type, t.1);

        let tileset = match self.tileset_quadrant(v) {
            1 => &mut self.tile_q1,
            2 => &mut self.tile_q2,
            3 => &mut self.tile_q3,
            4 => &mut self.tile_q4,
            _ => return,
        };

        let x_index = v.x.unsigned_abs() as usize;
        let y_index = v.y.unsigned_abs() as usize;
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

        let x_index = v.x.unsigned_abs() as usize;
        let y_index = v.y.unsigned_abs() as usize;
        tileset[x_index][y_index] = tile;
    }

    fn get_empty_flag(&mut self, v: IVec2) -> bool {
        self.get_tileset_raw(v).0 & EMPTY_TYPE_MASK == EMPTY_TYPE_MASK
    }

    fn get_water_flag(&mut self, v: IVec2) -> bool {
        self.get_tileset_raw(v).0 & WATER_TYPE_MASK == WATER_TYPE_MASK
    }

    fn set_water_flag(&mut self, v: IVec2) {
        self.set_type_index(v, WATER_TYPE_MASK);
    }

    fn get_grass_flag(&mut self, v: IVec2) -> bool {
        self.get_tileset_raw(v).0 & GRASS_TYPE_MASK == GRASS_TYPE_MASK
    }

    fn set_grass_flag(&mut self, v: IVec2) {
        self.set_type_index(v, GRASS_TYPE_MASK);
    }

    pub fn get_path_flag(&mut self, v: IVec2) -> bool {
        self.get_tileset_raw(v).0 & PATH_TYPE_MASK == PATH_TYPE_MASK
    }

    pub fn get_water_sparkle_flag(&mut self, v: IVec2) -> bool {
        if self.get_tileset_raw(v).0 & WATER_SPARKLE_TYPE_MASK != WATER_SPARKLE_TYPE_MASK {
            return false;
        }
        self.neigbhor_bitmask_grass(v) == 0
    }

    fn set_water_sparkle_flag(&mut self, v: IVec2) {
        self.set_type_index(v, WATER_SPARKLE_TYPE_MASK);
    }

    // Expand the tileset lists if the given point
    // lies outside the current range.
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
            if tileset.len() <= v.x.unsigned_abs() as usize {
                fitting_size = false;
                let mut addition = vec![
                    vec![(EMPTY_TYPE_MASK, INVALID_TILE); tileset[0].len()];
                    CHUNK_SIZE as usize
                ];
                tileset.append(&mut addition);
            }
            if tileset[0].len() <= v.y.unsigned_abs() as usize {
                fitting_size = false;
                for i in 0..tileset.len() {
                    let mut addition = vec![(EMPTY_TYPE_MASK, INVALID_TILE); tileset[0].len()];
                    tileset[i].append(&mut addition);
                }
            }
        }
    }

    fn water_height(&self, v: IVec2) -> f32 {
        let w = Vec2::new(v.x as f32, v.y as f32);

        let noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, self.seed());
        let secondary_noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, self.seed() + 1.0) * 1.0;
        noise + secondary_noise
    }

    fn water_sparkle_height(&self, v: IVec2) -> f32 {
        let w = Vec2::new(v.x as f32, v.y as f32);

        let noise = simplex_noise_2d_seeded(w * WATER_SPARKLE_NOISE_ZOOM, self.seed());
        let secondary_noise =
            simplex_noise_2d_seeded(w * WATER_SPARKLE_NOISE_ZOOM, self.seed() + 1.0) * 1.0;
        noise + secondary_noise
    }

    /// Determine if a given tile is water or grass.
    /// This will only set the water bit flag,
    /// not the actual tile index.
    fn collapse_water(&mut self, v: IVec2) -> bool {
        if !self.get_empty_flag(v) {
            return self.get_water_flag(v);
        }

        let height = self.water_height(v);
        let is_water = height < WATER_HEIGH_LEVEL;
        if is_water {
            self.set_water_flag(v);
            if !(WATER_SPARKLE_HEIGHT_LEVEL_MIN..=WATER_SPARKLE_HEIGHT_LEVEL_MAX).contains(&height)
                && self.water_sparkle_height(v) < 0.0
            {
                self.set_water_sparkle_flag(v);
            }
        } else {
            self.set_grass_flag(v);
        }
        is_water
    }

    fn get_flower_tile(&self, v: IVec2) -> u16 {
        let w = Vec2::new(v.x as f32, v.y as f32);

        let noise = simplex_noise_2d_seeded(w * FLOWER_NOISE_ZOOM, self.seed() + 2.0);
        let secondary_noise =
            simplex_noise_2d_seeded(w * FLOWER_NOISE_ZOOM, self.seed() + 3.0) * 1.0;
        let h = noise + secondary_noise;

        if h < FLOWER_HEIGHT_LEVEL {
            self.flower_mask.get_index(1)
        } else {
            self.flower_mask.get_index(0)
        }
    }

    /// Determine which tile to place. This will collapse
    /// the neigbhoring four tiles to see if they are grass or water.
    /// This is used for all tiles, both grass and water.
    fn collapse_tile(&mut self, v: IVec2) {
        let tile = match self.tile_type(v) {
            TileType::GrassWater => {
                let t = self.neigbhor_bitmask_grass(v);
                self.grass_mask.get_index(t)
            }
            TileType::PathOrGrass => {
                let t = self.neigbhor_bitmask_path(v);
                self.path_mask.get_index(t)
            }
        };

        if tile == GRASS_FLOWER_SUPER_POSITION {
            self.set_tileset(v, self.get_flower_tile(v));
        } else {
            self.set_tileset(v, tile);
        }
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

    /// Check if flora can be placed on the given tile.
    /// Flora can only be placed if the tile is surrounded by grass tiles.
    pub fn get_flora_flag(&mut self, v: IVec2) -> bool {
        self.get_grass_flag(v)
            && self.get_grass_flag(v + IVec2::X)
            && self.get_grass_flag(v + IVec2::Y)
            && self.get_grass_flag(v + IVec2::ONE)
    }

    pub fn seed(&self) -> f32 {
        self.seed
    }

    pub fn get_furthest_hotspots(&self, number_of_hotspots: usize) -> Vec<Vec2> {
        if number_of_hotspots >= self.vertices.len() {
            error!("Requesting more hotspots than exist in the bitmap! This should never happen. It means that you world proc gen isn't working properly");
            return self.vertices.clone();
        }

        let mut hotspots = self.vertices.clone();
        hotspots.sort_by(|a, b| a.length_squared().partial_cmp(&b.length_squared()).unwrap());
        let range = hotspots.len() - number_of_hotspots..hotspots.len();
        hotspots[range].to_vec()
    }

    pub fn set_vertices(&mut self, vertices: &Vec<Vec2>) {
        for v in vertices {
            self.vertices.push(*v * TILE_SIZE);
        }
    }

    pub fn set_edges(&mut self, edges: &HashSet<(usize, usize)>) {
        self.edges.clone_from(edges);
    }

    pub fn get_origin_edges(&self) -> Vec<Vec2> {
        self.edges
            .clone()
            .into_iter()
            .filter(|&(u, v)| u == 0 || v == 0)
            .map(|(u, v)| {
                if u == 0 {
                    self.vertices[v] - self.vertices[u]
                } else {
                    self.vertices[u] - self.vertices[v]
                }
            })
            .collect()
    }

    pub fn is_position_water(&mut self, pos: Vec2) -> bool {
        let v = IVec2::new((pos.x / TILE_SIZE) as i32, (pos.y / TILE_SIZE) as i32);
        self.get_water_flag(v)
            || self.get_water_flag(v - IVec2::X)
            || self.get_water_flag(v - IVec2::Y)
            || self.get_water_flag(v - IVec2::ONE)
            || self.get_water_flag(v + IVec2::new(-1, 1))
            || self.get_water_flag(v + IVec2::new(1, -1))
    }

    pub fn get_water_sparkle_indices(&mut self) -> Vec<u16> {
        self.water_sparkle_mask.get_animation_indices()
    }
}
