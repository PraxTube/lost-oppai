pub mod bitmap;

mod bitmask;
mod path;

pub use bitmap::BitMap;

use bevy::prelude::*;

use super::{CHUNK_SIZE, RENDERED_CHUNKS_RADIUS};

const NOISE_ZOOM: f32 = 0.02;
const FLOWER_NOISE_ZOOM: f32 = 0.1;
const WATER_SPARKLE_NOISE_ZOOM: f32 = 0.1;
// Determines the sea level,
// between -1 and 1, the water tiles must be below
// this value to count as water.
const WATER_HEIGH_LEVEL: f32 = -0.5;
const FLOWER_HEIGHT_LEVEL: f32 = -0.7;
const WATER_SPARKLE_HEIGHT_LEVEL_MAX: f32 = -0.6;
const WATER_SPARKLE_HEIGHT_LEVEL_MIN: f32 = -0.85;

const EMPTY_TYPE_MASK: u8 = 1 << 0;
const WATER_TYPE_MASK: u8 = 1 << 1;
const GRASS_TYPE_MASK: u8 = 1 << 2;
const PATH_TYPE_MASK: u8 = 1 << 3;
const WATER_SPARKLE_TYPE_MASK: u8 = 1 << 4;
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

#[derive(PartialEq)]
enum TileType {
    // This tile MUST be partly water or fully water
    GrassWater,
    // This tile is either full grass or full path or anything inbetween
    PathOrGrass,
}
