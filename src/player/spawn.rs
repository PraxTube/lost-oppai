use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::world::camera::YSort;
use crate::{GameAssets, GameState};

use super::{Player, PLAYER_SCALE, PLAYER_SPAWN_POS};

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
    let entity = commands
        .spawn((
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            YSort(0.0),
            SpriteSheetBundle {
                transform: Transform::from_translation(PLAYER_SPAWN_POS).with_scale(PLAYER_SCALE),
                texture_atlas: assets.player.clone(),
                ..default()
            },
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::ball(4.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::default(),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, -5.0, 0.0))),
        ))
        .id();

    commands
        .entity(entity)
        .insert(Player::new(collider))
        .push_children(&[collider]);
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_player);
    }
}
