use crate::{GameAssets, GameState};

use super::{
    generation::BitMap, BACKGROUND_ZINDEX_ABS, CHUNK_SIZE, RENDERED_CHUNKS_RADIUS, TILE_SIZE,
};
use bevy::{math::Vec3Swizzles, prelude::*, utils::HashSet};
use bevy_ecs_tilemap::prelude::*;

const RENDER_TILE_SIZE: TilemapTileSize = TilemapTileSize {
    x: TILE_SIZE.x,
    y: TILE_SIZE.y,
};
const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};

#[derive(Default, Debug, Resource)]
pub struct ChunkManager {
    spawned_chunks: HashSet<IVec2>,
}

#[derive(Component, Deref)]
pub struct ChunkIndex(pub IVec2);

fn spawn_chunk(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    map: &Res<BitMap>,
    chunk_pos: IVec2,
) {
    let tilemap_entity = commands.spawn(ChunkIndex(chunk_pos)).id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());

    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let v = IVec2::new(
                x as i32 + chunk_pos.x * CHUNK_SIZE.x as i32,
                y as i32 + chunk_pos.y * CHUNK_SIZE.y as i32,
            );
            let index = map.get(v) as u32;

            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(index),
                    ..Default::default()
                })
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * RENDER_TILE_SIZE.x,
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * RENDER_TILE_SIZE.y,
        -BACKGROUND_ZINDEX_ABS,
    ));

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: RENDER_TILE_SIZE.into(),
        size: CHUNK_SIZE.into(),
        storage: tile_storage,
        texture: TilemapTexture::Single(assets.tileset.clone()),
        tile_size: RENDER_TILE_SIZE,
        transform,
        render_settings: TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size = CHUNK_SIZE.as_ivec2();
    let tile_size = TILE_SIZE.as_ivec2();
    camera_pos / (chunk_size * tile_size)
}

pub fn spawn_chunks(
    mut commands: Commands,
    assets: Res<GameAssets>,
    camera_query: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
    map: Res<BitMap>,
) {
    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        info!(
            "pos: {}, chunk: {}",
            transform.translation.xy(),
            camera_chunk_pos
        );
        let chunks_radius =
            IVec2::new(RENDERED_CHUNKS_RADIUS as i32, RENDERED_CHUNKS_RADIUS as i32);

        for y in (camera_chunk_pos.y - chunks_radius.y)..=(camera_chunk_pos.y + chunks_radius.y) {
            for x in (camera_chunk_pos.x - chunks_radius.x)..=(camera_chunk_pos.x + chunks_radius.x)
            {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(&mut commands, &assets, &map, IVec2::new(x, y));
                }
            }
        }
    }
}

pub fn despawn_chunks(
    mut commands: Commands,
    q_camera: Query<&Transform, With<Camera>>,
    chunks_query: Query<(Entity, &ChunkIndex)>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let camera_transform = match q_camera.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    let camera_chunk = camera_pos_to_chunk_pos(&camera_transform.translation.xy());

    for (entity, chunk) in &chunks_query {
        if (chunk.x - camera_chunk.x).abs() as u32 > RENDERED_CHUNKS_RADIUS
            || (chunk.y - camera_chunk.y).abs() as u32 > RENDERED_CHUNKS_RADIUS
        {
            chunk_manager
                .spawned_chunks
                .remove(&IVec2::new(chunk.x, chunk.y));
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct MapRenderPlugin;

impl Plugin for MapRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .insert_resource(ChunkManager::default())
            .add_systems(
                Update,
                (
                    spawn_chunks.run_if(resource_exists::<BitMap>()),
                    despawn_chunks,
                )
                    .run_if(in_state(GameState::Gaming)),
            );
    }
}
