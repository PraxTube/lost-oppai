use bevy::prelude::*;
use bevy::utils::HashSet;
use noisy_bevy::simplex_noise_2d_seeded;

use crate::world::map::TILE_SIZE;

use super::bitmask::{BitMasks, GRASS_FLOWER_SUPER_POSITION};
use super::{
    TileCollision, TileType, BITMASK_BOT_LEFT, BITMASK_BOT_RIGHT, BITMASK_TOP_LEFT,
    BITMASK_TOP_RIGHT, CHUNK_SIZE, EMPTY_TYPE_INDEX, FLOWER_HEIGHT_LEVEL, FLOWER_NOISE_ZOOM,
    GRASS_TYPE_INDEX, INVALID_TILE, NOISE_ZOOM, PATH_TYPE_INDEX, RENDERED_CHUNKS_RADIUS,
    WATER_HEIGH_LEVEL, WATER_TYPE_INDEX,
};

#[derive(Resource)]
pub struct BitMap {
    seed: f32,
    vertices: Vec<Vec2>,
    edges: HashSet<(usize, usize)>,
    tile_q1: Vec<Vec<(u8, u16)>>,
    tile_q2: Vec<Vec<(u8, u16)>>,
    tile_q3: Vec<Vec<(u8, u16)>>,
    tile_q4: Vec<Vec<(u8, u16)>>,
}

impl Default for BitMap {
    fn default() -> Self {
        let length = CHUNK_SIZE as usize * RENDERED_CHUNKS_RADIUS as usize;
        Self {
            seed: 61.0,
            vertices: Vec::new(),
            edges: HashSet::new(),
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

        let x_index = v.x.abs() as usize;
        let y_index = v.y.abs() as usize;
        tileset[x_index][y_index]
    }

    fn get_tileset(&mut self, v: IVec2) -> u16 {
        self.get_tileset_raw(v).1
    }

    pub fn set_type_index(&mut self, v: IVec2, tile_type: u8) {
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
        } else {
            warn!("Trying to set water flag on non empty type tile");
        }
    }

    fn set_grass_flag(&mut self, v: IVec2) {
        if self.get_tileset_raw(v).0 == EMPTY_TYPE_INDEX {
            self.set_type_index(v, GRASS_TYPE_INDEX);
        } else {
            warn!("Trying to set grass flag on non empty type tile");
        }
    }

    pub fn get_path_flag(&mut self, v: IVec2) -> bool {
        self.get_tileset_raw(v).0 == PATH_TYPE_INDEX
    }

    // Expand the tileset arrays if the given point
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

        let noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, self.seed());
        let secondary_noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, self.seed() + 1.0) * 1.0;
        let h = noise + secondary_noise;

        let is_water = h < WATER_HEIGH_LEVEL;
        if is_water {
            self.set_water_flag(v);
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
            BitMasks::flower().get_index(1)
        } else {
            BitMasks::flower().get_index(0)
        }
    }

    /// Determine which tile to place. This will collapse
    /// the neigbhoring four tiles to see if they are grass or water.
    /// This is used for all tiles, both grass and water.
    fn collapse_tile(&mut self, v: IVec2) {
        let (mask, bitmask) = match self.tile_type(v) {
            TileType::GrassWater => (self.neigbhor_bitmask_grass(v), BitMasks::grass()),
            TileType::PathOrGrass => (self.neigbhor_bitmask_path(v), BitMasks::path()),
        };

        let tile = bitmask.get_index(mask);
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
        self.get_tileset_raw(v).0 == GRASS_TYPE_INDEX
            && self.get_tileset_raw(v + IVec2::X).0 == GRASS_TYPE_INDEX
            && self.get_tileset_raw(v + IVec2::Y).0 == GRASS_TYPE_INDEX
            && self.get_tileset_raw(v + IVec2::ONE).0 == GRASS_TYPE_INDEX
    }

    pub fn seed(&self) -> f32 {
        self.seed
    }

    pub fn get_hotspot(&self, index: usize) -> Vec2 {
        if index >= self.vertices.len() {
            return Vec2::ZERO;
        }

        self.vertices[index]
    }

    pub fn set_vertices(&mut self, vertices: &Vec<Vec2>) {
        for v in vertices {
            self.vertices.push(*v * TILE_SIZE);
        }
    }

    pub fn set_edges(&mut self, edges: &HashSet<(usize, usize)>) {
        self.edges = edges.clone();
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
}
