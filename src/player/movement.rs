use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GameState;

use super::chat::PlayerStartedChat;
use super::input::PlayerInput;
use super::{Player, PlayerState, RUN_SPEED, WALK_SPEED};

fn player_movement(
    player_input: Res<PlayerInput>,
    mut q_player: Query<(&mut Velocity, &mut Player)>,
) {
    let (mut velocity, mut player) = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    if player.state == PlayerState::Talking {
        velocity.linvel = Vec2::ZERO;
        return;
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

fn face_npc(
    mut q_player: Query<&mut Player>,
    mut ev_player_started_chat: EventReader<PlayerStartedChat>,
) {
    for ev in ev_player_started_chat.read() {
        let mut player = match q_player.get_single_mut() {
            Ok(p) => p,
            Err(_) => return,
        };

        player.current_direction = ev.direction;
    }
}

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_movement, face_npc).run_if(in_state(GameState::Gaming)),
        );
    }
}
