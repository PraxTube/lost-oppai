use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use super::{input::PlayerInput, Player};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum PlayerState {
    #[default]
    Idling,
    Walking,
    Running,
}

#[derive(Event)]
pub struct PlayerChangedState {
    pub old_state: PlayerState,
    pub new_state: PlayerState,
}

fn player_changed_state(
    q_player: Query<&Player>,
    mut ev_changed_state: EventWriter<PlayerChangedState>,
    mut old_state: Local<PlayerState>,
) {
    let player = match q_player.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };

    if player.state != *old_state {
        ev_changed_state.send(PlayerChangedState {
            old_state: *old_state,
            new_state: player.state,
        });
        *old_state = player.state;
    }
}

fn switch_player_mode(
    player_input: Res<PlayerInput>,
    mut q_player: Query<(&Velocity, &mut Player)>,
) {
    let (velocity, mut player) = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    let state = if velocity.linvel == Vec2::ZERO {
        PlayerState::Idling
    } else {
        if player_input.running {
            PlayerState::Running
        } else {
            PlayerState::Walking
        }
    };
    player.state = state;
}

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                switch_player_mode,
                player_changed_state.after(switch_player_mode),
            ),
        )
        .add_event::<PlayerChangedState>();
    }
}
