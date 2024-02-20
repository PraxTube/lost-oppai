pub mod input;
pub mod state;

// mod audio;
mod collision;
mod movement;
mod spawn;

// pub use state::PlayerChangedState;
pub use state::PlayerState;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const PLAYER_SPAWN_POS: Vec3 = Vec3::ZERO;
const PLAYER_SCALE: Vec3 = Vec3::splat(0.5);
const WALK_SPEED: f32 = 75.0;
const RUN_SPEED: f32 = 130.0;
const PLAYER_COLLISION_GROUPS: CollisionGroups = CollisionGroups::new(Group::ALL, Group::ALL);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            input::InputPlugin,
            state::PlayerStatePlugin,
            // audio::PlayerAudioPlugin,
            spawn::PlayerSpawnPlugin,
            collision::PlayerCollisionPlugin,
            movement::PlayerMovementPlugin,
        ));
    }
}

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
    pub current_direction: Vec2,
    pub collider_entity: Entity,
}

impl Player {
    fn new(collider_entity: Entity) -> Self {
        Self {
            state: PlayerState::default(),
            current_direction: Vec2::ZERO,
            collider_entity,
        }
    }
}
