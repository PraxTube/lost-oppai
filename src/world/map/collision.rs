use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{
    generation::{BitMap, TileCollision},
    render::{despawn_chunks, spawn_chunks, ChunkIndex},
    CHUNK_SIZE, TILE_SIZE,
};

const RECT_WIDTH: f32 = 1.0;

fn bot_left_tri() -> Collider {
    match Collider::convex_hull(&[
        Vect::new(-TILE_SIZE / 2.0, -TILE_SIZE / 2.0),
        Vect::new(-TILE_SIZE / 2.0, TILE_SIZE / 2.0),
        Vect::new(TILE_SIZE / 2.0, -TILE_SIZE / 2.0),
    ]) {
        Some(r) => r,
        None => Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
    }
}

fn top_left_tri() -> Collider {
    match Collider::convex_hull(&[
        Vect::new(-TILE_SIZE / 2.0, TILE_SIZE / 2.0),
        Vect::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
        Vect::new(-TILE_SIZE / 2.0, -TILE_SIZE / 2.0),
    ]) {
        Some(r) => r,
        None => Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
    }
}

fn top_right_tri() -> Collider {
    match Collider::convex_hull(&[
        Vect::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
        Vect::new(TILE_SIZE / 2.0, -TILE_SIZE / 2.0),
        Vect::new(-TILE_SIZE / 2.0, TILE_SIZE / 2.0),
    ]) {
        Some(r) => r,
        None => Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
    }
}

fn bot_right_tri() -> Collider {
    match Collider::convex_hull(&[
        Vect::new(TILE_SIZE / 2.0, -TILE_SIZE / 2.0),
        Vect::new(-TILE_SIZE / 2.0, -TILE_SIZE / 2.0),
        Vect::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
    ]) {
        Some(r) => r,
        None => Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
    }
}

fn spawn_water_collision(
    commands: &mut Commands,
    map: &mut ResMut<BitMap>,
    entity: &Entity,
    chunk: &ChunkIndex,
    x: u32,
    y: u32,
) {
    let v = IVec2::new(
        x as i32 + chunk.x * CHUNK_SIZE as i32,
        y as i32 + chunk.y * CHUNK_SIZE as i32,
    );
    let (collider, offset) = match map.get_tile_collision(v) {
        TileCollision::BotRect => (
            Collider::cuboid(TILE_SIZE / 2.0, RECT_WIDTH),
            Vec3::new(0.0, -TILE_SIZE / 2.0 - RECT_WIDTH, 0.0),
        ),
        TileCollision::LeftRect => (
            Collider::cuboid(RECT_WIDTH, TILE_SIZE / 2.0),
            Vec3::new(-TILE_SIZE / 2.0 - RECT_WIDTH, 0.0, 0.0),
        ),
        TileCollision::TopRect => (
            Collider::cuboid(TILE_SIZE / 2.0, RECT_WIDTH),
            Vec3::new(0.0, TILE_SIZE / 2.0 + RECT_WIDTH, 0.0),
        ),
        TileCollision::RightRect => (
            Collider::cuboid(RECT_WIDTH, TILE_SIZE / 2.0),
            Vec3::new(TILE_SIZE / 2.0 + RECT_WIDTH, 0.0, 0.0),
        ),
        TileCollision::BotLeftTri => (bot_left_tri(), Vec3::ZERO),
        TileCollision::TopLeftTri => (top_left_tri(), Vec3::ZERO),
        TileCollision::TopRightTri => (top_right_tri(), Vec3::ZERO),
        TileCollision::BotRightTri => (bot_right_tri(), Vec3::ZERO),
        TileCollision::None => return,
    };

    let pos = Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0);
    let collision = commands
        .spawn((
            collider,
            TransformBundle::from_transform(Transform::from_translation(pos + offset)),
        ))
        .id();
    commands.entity(*entity).push_children(&[collision]);
}

pub fn spawn_water_collisions(
    mut commands: Commands,
    mut map: ResMut<BitMap>,
    q_chunks: Query<(Entity, &ChunkIndex), Added<ChunkIndex>>,
) {
    for (entity, chunk) in &q_chunks {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                spawn_water_collision(&mut commands, &mut map, &entity, chunk, x, y);
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
