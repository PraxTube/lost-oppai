pub mod generation;

mod collision;
mod flora;
mod poisson_sampling;
mod render;

use bevy::prelude::*;

// Values based on the used tileset, don't change!
const TILE_SIZE: f32 = 16.0;
const CHUNK_SIZE: u32 = 16;
const BACKGROUND_ZINDEX_ABS: f32 = 800.0;

const RENDERED_CHUNKS_RADIUS: u32 = 3;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            generation::MapGenerationPlugin,
            render::MapRenderPlugin,
            collision::MapCollisionPlugin,
            flora::FloraPlugin,
        ));
    }
}
