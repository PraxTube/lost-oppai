use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::{dynamics::Velocity, plugin::PhysicsSet};
use bevy_trickfilm::prelude::*;
use bevy_yarnspinner::events::DialogueCompleteEvent;

use crate::{GameAssets, GameState};

use super::{chat::PlayerStoppedChat, input::PlayerInput, Player};

// Used to make sure we always play up/down animation when player moves diagonally.
const MOVEMENT_ANIMATION_ANGLE_BUFFER: f32 = 0.05;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum PlayerState {
    #[default]
    Idling,
    Walking,
    Running,
    Talking,
}

fn switch_to_idling_from_talking(mut q_player: Query<&mut Player>) {
    let mut player = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    player.state = PlayerState::Idling;
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
    mut q_player: Query<(&mut AnimationPlayer2D, &Player)>,
) {
    let (mut animator, player) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let dir = if player.current_direction == Vec2::ZERO {
        Vec2::NEG_X
    } else {
        player.current_direction
    };

    let angle = Vec2::X.angle_between(dir);
    let direction_index = if (-3.0 / 4.0 * PI - MOVEMENT_ANIMATION_ANGLE_BUFFER
        ..=-1.0 / 4.0 * PI + MOVEMENT_ANIMATION_ANGLE_BUFFER)
        .contains(&angle)
    {
        // Down
        0
    } else if (1.0 / 4.0 * PI - MOVEMENT_ANIMATION_ANGLE_BUFFER
        ..=3.0 / 4.0 * PI + MOVEMENT_ANIMATION_ANGLE_BUFFER)
        .contains(&angle)
    {
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
                switch_to_idling_from_talking.run_if(
                    on_event::<PlayerStoppedChat>().or_else(on_event::<DialogueCompleteEvent>()),
                ),
                switch_player_move_state,
                update_animation,
            )
                .chain()
                .before(PhysicsSet::Writeback)
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
