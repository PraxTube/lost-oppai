use crate::{world::MainCamera, GameAssets, GameState};

use super::{
    generation::BitMap, BACKGROUND_ZINDEX_ABS, CHUNK_SIZE, RENDERED_CHUNKS_RADIUS, TILE_SIZE,
};
use bevy::{math::Vec3Swizzles, prelude::*, utils::HashSet};
use bevy_ecs_tilemap::prelude::*;

const RENDER_TILE_SIZE: TilemapTileSize = TilemapTileSize {
    x: TILE_SIZE,
    y: TILE_SIZE,
};
const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE as u32 * 2,
    y: CHUNK_SIZE as u32 * 2,
};

#[derive(Default, Debug, Resource)]
pub struct ChunkManager {
    spawned_chunks: HashSet<IVec2>,
}

#[derive(Component, Deref)]
pub struct ChunkIndex(pub IVec2);

#[derive(Event)]
pub struct SpawnedChunk {
    pub pos: IVec2,
}

#[derive(Event)]
pub struct DespawnedChunk {
    pub chunk_pos: IVec2,
}

fn spawn_chunk(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    map: &mut ResMut<BitMap>,
    chunk_pos: IVec2,
) {
    let tilemap_entity = commands.spawn(ChunkIndex(chunk_pos)).id();
    let mut tile_storage = TileStorage::empty(TilemapSize::new(CHUNK_SIZE, CHUNK_SIZE));

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let v = IVec2::new(
                x as i32 + chunk_pos.x * CHUNK_SIZE as i32,
                y as i32 + chunk_pos.y * CHUNK_SIZE as i32,
            );
            let index = map.get_tile_index(v) as u32;

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
        chunk_pos.x as f32 * CHUNK_SIZE as f32 * RENDER_TILE_SIZE.x,
        chunk_pos.y as f32 * CHUNK_SIZE as f32 * RENDER_TILE_SIZE.y,
        -BACKGROUND_ZINDEX_ABS,
    ));

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: RENDER_TILE_SIZE.into(),
        size: TilemapSize::new(CHUNK_SIZE, CHUNK_SIZE),
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
    let chunk_size = IVec2::ONE * CHUNK_SIZE as i32;
    let tile_size = IVec2::ONE * TILE_SIZE as i32;

    let offset = IVec2::new(
        if camera_pos.x < 0 { -1 } else { 0 },
        if camera_pos.y < 0 { -1 } else { 0 },
    );

    camera_pos / (chunk_size * tile_size) + offset
}

pub fn spawn_chunks(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut bitmap: ResMut<BitMap>,
    q_camera: Query<&Transform, With<MainCamera>>,
    mut ev_spawned_chunk: EventWriter<SpawnedChunk>,
) {
    let camera_transform = match q_camera.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    let camera_chunk_pos = camera_pos_to_chunk_pos(&camera_transform.translation.xy());
    let chunks_radius = IVec2::new(RENDERED_CHUNKS_RADIUS as i32, RENDERED_CHUNKS_RADIUS as i32);

    for y in (camera_chunk_pos.y - chunks_radius.y)..=(camera_chunk_pos.y + chunks_radius.y) {
        for x in (camera_chunk_pos.x - chunks_radius.x)..=(camera_chunk_pos.x + chunks_radius.x) {
            if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                let chunk_pos = IVec2::new(x, y);
                chunk_manager.spawned_chunks.insert(chunk_pos);
                spawn_chunk(&mut commands, &assets, &mut bitmap, chunk_pos);
                ev_spawned_chunk.send(SpawnedChunk { pos: chunk_pos });
            }
        }
    }
}

pub fn despawn_chunks(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    q_camera: Query<&Transform, With<MainCamera>>,
    chunks_query: Query<(Entity, &ChunkIndex)>,
    mut ev_despawned_chunk: EventWriter<DespawnedChunk>,
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
            let chunk_pos = IVec2::new(chunk.x, chunk.y);
            chunk_manager.spawned_chunks.remove(&chunk_pos);
            ev_despawned_chunk.send(DespawnedChunk { chunk_pos });
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct ChunkManagerPlugin;

impl Plugin for ChunkManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .insert_resource(ChunkManager::default())
            .add_event::<SpawnedChunk>()
            .add_event::<DespawnedChunk>()
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
