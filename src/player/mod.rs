pub mod input;
pub mod state;

// mod audio;
mod movement;
mod spawn;

// pub use state::PlayerChangedState;
pub use state::PlayerState;

use bevy::prelude::*;

pub const PLAYER_SPAWN_POS: Vec3 = Vec3::ZERO;
const PLAYER_SCALE: Vec3 = Vec3::splat(0.5);
const MOVE_SPEED: f32 = 150.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            input::InputPlugin,
            state::PlayerStatePlugin,
            // audio::PlayerAudioPlugin,
            movement::PlayerMovementPlugin,
            spawn::PlayerSpawnPlugin,
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
