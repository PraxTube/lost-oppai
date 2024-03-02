pub mod bitmap;

mod bitmask;
mod path;

pub use bitmap::BitMap;

use bevy::prelude::*;

use super::{CHUNK_SIZE, RENDERED_CHUNKS_RADIUS};

const NOISE_ZOOM: f32 = 0.02;

const EMPTY_TYPE_INDEX: u8 = 0;
const WATER_TYPE_INDEX: u8 = 1;
const GRASS_TYPE_INDEX: u8 = 2;
const PATH_TYPE_INDEX: u8 = 3;
const INVALID_TILE: u16 = 15 * 16;

const BITMASK_TOP_RIGHT: u16 = 1 << 0;
const BITMASK_BOT_RIGHT: u16 = 1 << 1;
const BITMASK_BOT_LEFT: u16 = 1 << 2;
const BITMASK_TOP_LEFT: u16 = 1 << 3;

pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(path::PathGenerationPlugin)
            .init_resource::<BitMap>();
    }
}

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
