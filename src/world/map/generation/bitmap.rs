use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d_seeded;

use super::bitmask::BitMasks;
use super::{
    TileCollision, TileType, BITMASK_BOT_LEFT, BITMASK_BOT_RIGHT, BITMASK_TOP_LEFT,
    BITMASK_TOP_RIGHT, CHUNK_SIZE, EMPTY_TYPE_INDEX, INVALID_TILE, NOISE_ZOOM, PATH_TYPE_INDEX,
    RENDERED_CHUNKS_RADIUS, WATER_TYPE_INDEX,
};

#[derive(Resource)]
pub struct BitMap {
    seed: f32,
    center_point: Vec2,
    tile_q1: Vec<Vec<(u8, u16)>>,
    tile_q2: Vec<Vec<(u8, u16)>>,
    tile_q3: Vec<Vec<(u8, u16)>>,
    tile_q4: Vec<Vec<(u8, u16)>>,
}

impl Default for BitMap {
    fn default() -> Self {
        let length = CHUNK_SIZE as usize * RENDERED_CHUNKS_RADIUS as usize;
        Self {
            seed: 60.0,
            center_point: Vec2::ZERO,
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

    pub fn get_tileset_raw(&mut self, v: IVec2) -> (u8, u16) {
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

        let noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, self.seed());
        let secondary_noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, self.seed() + 1.0) * 1.0;
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

    pub fn seed(&self) -> f32 {
        self.seed
    }

    pub fn center_point(&self) -> Vec2 {
        self.center_point
    }

    pub fn set_center_point(&mut self, p: Vec2) {
        self.center_point = p;
    }
}
