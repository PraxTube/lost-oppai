use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GameState;

use super::input::PlayerInput;
use super::{Player, PlayerState, RUN_SPEED, WALK_SPEED};

fn player_movement(
    mut q_player: Query<(&mut Velocity, &mut Player)>,
    player_input: Res<PlayerInput>,
) {
    let (mut velocity, mut player) = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    let direction = player_input.move_direction;
    if direction == Vec2::default() {
        velocity.linvel = Vec2::ZERO;
        return;
    }

    let speed = if player.state == PlayerState::Running {
        RUN_SPEED
    } else {
        WALK_SPEED
    };

    player.current_direction = direction;
    velocity.linvel = direction * speed;
}

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_movement,).run_if(in_state(GameState::Gaming)),
        );
    }
}
