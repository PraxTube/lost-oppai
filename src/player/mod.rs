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

use crate::utils::FixedQueue;

pub const PLAYER_SPAWN_POS: Vec3 = Vec3::ZERO;
pub const NPC_PROXIMITY_DISTANCE: f32 = 50.0;
const PLAYER_SCALE: Vec3 = Vec3::splat(0.5);
const WALK_SPEED: f32 = 75.0;
const RUN_SPEED: f32 = 115.0;
const PLAYER_COLLISION_GROUPS: CollisionGroups = CollisionGroups::new(Group::ALL, Group::ALL);

const DIRECTIONS_QUEUE_SIZE: usize = 100;

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
    directions_queue: FixedQueue<Vec2, DIRECTIONS_QUEUE_SIZE>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            state: PlayerState::default(),
            current_direction: Vec2::NEG_Y,
            directions_queue: FixedQueue::new(),
        }
    }
}

impl Player {
    pub fn average_direction(&self) -> Vec2 {
        let dir = self.directions_queue.compute_average();

        match dir {
            Some(r) => r / DIRECTIONS_QUEUE_SIZE as f32,
            None => Vec2::NEG_Y,
        }
    }
}
