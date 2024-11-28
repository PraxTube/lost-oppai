pub mod chat;
pub mod input;
pub mod state;

mod audio;
mod collision;
mod movement;
mod spawn;

pub use state::PlayerState;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const PLAYER_SPAWN_POS: Vec3 = Vec3::ZERO;
pub const NPC_PROXIMITY_DISTANCE: f32 = 50.0;
const PLAYER_SCALE: Vec3 = Vec3::splat(0.5);
const WALK_SPEED: f32 = 100.0;
const RUN_SPEED: f32 = 150.0;
const PLAYER_COLLISION_GROUPS: CollisionGroups = CollisionGroups::new(Group::ALL, Group::ALL);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            collision::PlayerCollisionPlugin,
            input::InputPlugin,
            state::PlayerStatePlugin,
            audio::PlayerAudioPlugin,
            chat::PlayerChatPlugin,
            spawn::PlayerSpawnPlugin,
            movement::PlayerMovementPlugin,
        ));
    }
}

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
    pub current_direction: Vec2,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            state: PlayerState::default(),
            current_direction: Vec2::NEG_Y,
        }
    }
}
