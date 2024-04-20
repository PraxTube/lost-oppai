use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{world::camera::YSort, GameAssets, GameState};

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
    spawn_tree(&mut commands, &assets, Vec3::ONE);
}

pub struct FloraPlugin;

impl Plugin for FloraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), (spawn_flora,));
    }
}
