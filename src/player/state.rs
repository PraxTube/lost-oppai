use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;
use bevy_trickfilm::prelude::*;
use bevy_yarnspinner::events::DialogueCompleteEvent;

use crate::{GameAssets, GameState};

use super::{
    chat::{PlayerStartedChat, PlayerStoppedChat},
    input::PlayerInput,
    Player,
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum PlayerState {
    #[default]
    Idling,
    Walking,
    Running,
    Talking,
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

fn switch_to_talking(
    mut q_player: Query<&mut Player>,
    mut ev_player_started_chat: EventReader<PlayerStartedChat>,
) {
    if ev_player_started_chat.is_empty() {
        return;
    }
    ev_player_started_chat.clear();

    let mut player = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    player.state = PlayerState::Talking;
}

fn switch_away_talking(
    mut q_player: Query<&mut Player>,
    mut ev_dialogue_complete: EventReader<DialogueCompleteEvent>,
    mut ev_player_stopped_chat: EventReader<PlayerStoppedChat>,
) {
    if ev_player_stopped_chat.is_empty() && ev_dialogue_complete.is_empty() {
        return;
    }
    ev_player_stopped_chat.clear();
    ev_dialogue_complete.clear();

    let mut player = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if player.state == PlayerState::Talking {
        player.state = PlayerState::Idling;
    }
}

fn switch_player_move_state(
    player_input: Res<PlayerInput>,
    mut q_player: Query<(&Velocity, &mut Player)>,
) {
    let (velocity, mut player) = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    if player.state == PlayerState::Talking {
        return;
    }

    let state = if velocity.linvel == Vec2::ZERO {
        PlayerState::Idling
    } else if player_input.running {
        PlayerState::Running
    } else {
        PlayerState::Walking
    };
    player.state = state;
}

fn update_animation(
    assets: Res<GameAssets>,
    mut q_player: Query<(&Velocity, &mut AnimationPlayer2D, &Player)>,
) {
    let (velocity, mut animator, player) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let dir = if velocity.linvel == Vec2::ZERO {
        if player.current_direction == Vec2::ZERO {
            Vec2::NEG_X
        } else {
            player.current_direction
        }
    } else {
        velocity.linvel
    };

    let angle = Vec2::X.angle_between(dir);
    let direction_index = if (-3.0 / 4.0 * PI..=-1.0 / 4.0 * PI).contains(&angle) {
        // Down
        0
    } else if (1.0 / 4.0 * PI..=3.0 / 4.0 * PI).contains(&angle) {
        // Up
        3
    } else if !(-3.0 / 4.0 * PI..=3.0 / 4.0 * PI).contains(&angle) {
        // Left
        1
    } else {
        // Right
        2
    };

    let (clip, repeat) = match player.state {
        PlayerState::Idling => (assets.player_animations[direction_index].clone(), true),
        PlayerState::Walking => (assets.player_animations[4 + direction_index].clone(), true),
        PlayerState::Running => (assets.player_animations[8 + direction_index].clone(), true),
        PlayerState::Talking => (assets.player_animations[direction_index].clone(), true),
    };

    if repeat {
        animator.play(clip).repeat();
    } else {
        animator.play(clip);
    }
}

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                switch_to_talking.before(switch_player_move_state),
                switch_away_talking,
                switch_player_move_state,
                update_animation.after(switch_player_move_state),
                player_changed_state.after(switch_player_move_state),
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<PlayerChangedState>();
    }
}
