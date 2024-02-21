use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{
    generation::BitMap,
    render::{despawn_chunks, spawn_chunks, ChunkIndex},
    CHUNK_SIZE, TILE_SIZE,
};

pub fn spawn_water_collisions(
    mut commands: Commands,
    mut map: ResMut<BitMap>,
    q_chunks: Query<(Entity, &ChunkIndex), Added<ChunkIndex>>,
) {
    for (entity, chunk) in &q_chunks {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let v = IVec2::new(
                    x as i32 + chunk.x * CHUNK_SIZE as i32,
                    y as i32 + chunk.y * CHUNK_SIZE as i32,
                );
                if map.is_collision(v) {
                    let collision = commands
                        .spawn((
                            Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
                            TransformBundle::from_transform(Transform::from_translation(
                                Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0),
                            )),
                        ))
                        .id();
                    commands.entity(entity).push_children(&[collision]);
                }
            }
        }
    }
}

pub struct MapCollisionPlugin;

impl Plugin for MapCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_water_collisions,)
                .before(spawn_chunks)
                .before(despawn_chunks),
        );
    }
}
