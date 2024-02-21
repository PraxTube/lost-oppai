mod collision;
mod generation;
mod render;

use bevy::prelude::*;

const TILE_SIZE: f32 = 16.0;
const CHUNK_SIZE: u32 = 16;
const RENDERED_CHUNKS_RADIUS: u32 = 3;
const BACKGROUND_ZINDEX_ABS: f32 = 800.0;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            generation::MapGenerationPlugin,
            render::MapRenderPlugin,
            collision::MapCollisionPlugin,
        ));
    }
}
