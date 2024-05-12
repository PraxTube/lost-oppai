use rand::{thread_rng, Rng};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{world::camera::YSort, GameAssets, GameState};

use super::{
    chunk_manager::SpawnedChunk, generation::BitMap,
    poisson_sampling::generate_poisson_points_variable_radii, CHUNK_SIZE, TILE_SIZE,
};

const ROCKS_COUNT: usize = 3;

const REJECTION_ITER: usize = 20;
const MIN_RADIUS: f32 = 1.0;
const MAX_RADIUS: f32 = 4.0;

const TREE_RADIUS: f32 = 3.0;
const BUSH_RADIUS: f32 = 1.25;
const ROCK_RADIUS: f32 = 1.0;

fn spawn_rock(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3) {
    let collider = commands
        .spawn((
            Collider::cuboid(8.0, 8.0),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))),
        ))
        .id();

    let mut rng = thread_rng();

    commands
        .spawn((
            YSort(-40.0),
            SpriteSheetBundle {
                transform: Transform::from_translation(pos),
                texture_atlas: assets.rocks.clone(),
                sprite: TextureAtlasSprite {
                    index: rng.gen_range(0..ROCKS_COUNT),
                    ..default()
                },
                ..default()
            },
        ))
        .push_children(&[collider]);
}

fn spawn_bush(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3) {
    let base = commands
        .spawn((SpriteBundle {
            texture: assets.bush_base.clone(),
            ..default()
        },))
        .id();

    let collider = commands
        .spawn((
            Collider::cuboid(16.0, 8.0),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, -8.0, 0.0))),
        ))
        .id();

    commands
        .spawn((
            YSort(0.0),
            SpriteBundle {
                transform: Transform::from_translation(pos),
                texture: assets.bush.clone(),
                ..default()
            },
        ))
        .push_children(&[base, collider]);
}

fn spawn_tree(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3) {
    let pos = pos + Vec3::new(0.0, 48.0, 0.0);

    let trunk = commands
        .spawn((
            YSort(-20.0),
            SpriteBundle {
                texture: assets.tree_trunk.clone(),
                ..default()
            },
        ))
        .id();

    let shadow = commands
        .spawn((
            YSort(300.0),
            SpriteBundle {
                texture: assets.tree_shadow.clone(),
                ..default()
            },
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::cuboid(16.0, 8.0),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -48.0, 0.0,
            ))),
        ))
        .id();

    commands
        .spawn((
            YSort(40.0),
            SpriteBundle {
                transform: Transform::from_translation(pos),
                texture: assets.tree.clone(),
                ..default()
            },
        ))
        .push_children(&[trunk, shadow, collider]);
}

fn spawn_flora(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    bitmap: &mut ResMut<BitMap>,
    chunk_pos: IVec2,
    pos: Vec2,
    radius: f32,
) {
    let v = IVec2::new(
        pos.x.floor() as i32 + chunk_pos.x * CHUNK_SIZE as i32,
        pos.y.floor() as i32 + chunk_pos.y * CHUNK_SIZE as i32,
    );
    let pos = TILE_SIZE * Vec3::new(v.x as f32, v.y as f32, 0.0);

    if radius > TREE_RADIUS {
        if !(bitmap.get_flora_flag(v)
            && bitmap.get_flora_flag(v + IVec2::new(1, 0))
            && bitmap.get_flora_flag(v + IVec2::new(0, 1))
            && bitmap.get_flora_flag(v + IVec2::new(1, 1))
            && bitmap.get_flora_flag(v + IVec2::new(2, 0))
            && bitmap.get_flora_flag(v + IVec2::new(-2, 0)))
        {
            return;
        }
        spawn_tree(commands, assets, pos);
    } else if radius > BUSH_RADIUS {
        if !bitmap.get_flora_flag(v) {
            return;
        }
        spawn_bush(commands, assets, pos);
    } else if radius > ROCK_RADIUS {
        if !bitmap.get_flora_flag(v) {
            return;
        }
        spawn_rock(commands, assets, pos);
    } else {
    }
}

fn spawn_flora_chunk(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut bitmap: ResMut<BitMap>,
    mut ev_spawned_chunk: EventReader<SpawnedChunk>,
) {
    for ev in ev_spawned_chunk.read() {
        let seed = bitmap.seed() as u64 + ev.pos.x.abs() as u64 + ev.pos.y.abs() as u64;
        let points_with_radius = generate_poisson_points_variable_radii(
            MIN_RADIUS,
            MAX_RADIUS,
            CHUNK_SIZE as f32 * Vec2::ONE,
            REJECTION_ITER,
            seed,
        );
        for p_r in points_with_radius {
            let (x, y, r) = (p_r.x, p_r.y, p_r.z);
            let point = Vec2::new(x, y);
            spawn_flora(&mut commands, &assets, &mut bitmap, ev.pos, point, r);
        }
    }
}

pub struct FloraPlugin;

impl Plugin for FloraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_flora_chunk,).run_if(in_state(GameState::Gaming)),
        );
    }
}