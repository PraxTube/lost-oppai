mod generation;
mod render;

use bevy::prelude::*;

const CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };
const RENDERED_CHUNKS: f32 = 5.0;
const BACKGROUND_ZINDEX_ABS: f32 = 800.0;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((generation::MapGenerationPlugin, render::MapRenderPlugin));
    }
}
