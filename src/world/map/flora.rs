use rand::{thread_rng, Rng};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{world::camera::YSort, GameAssets, GameState};

const ROCKS_COUNT: usize = 3;

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

fn spawn_flora(mut commands: Commands, assets: Res<GameAssets>) {
    spawn_rock(&mut commands, &assets, Vec3::new(-100.0, -100.0, 0.0));
    spawn_tree(&mut commands, &assets, Vec3::ONE);
    spawn_bush(&mut commands, &assets, Vec3::new(-100.0, 100.0, 0.0));
}

pub struct FloraPlugin;

impl Plugin for FloraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), (spawn_flora,));
    }
}
