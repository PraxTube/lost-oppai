use std::time::Duration;

use bevy::prelude::*;

use crate::{audio::PlaySound, GameAssets, GameState};

use super::{Player, PlayerState};

const TIME_BETWEEN_STEPS_WALKING: f32 = 0.5;
const TIME_BETWEEN_STEPS_RUNNING: f32 = 0.4;
const RAND_SPEED_INTENSITY: f64 = 0.2;

#[derive(Resource, Deref, DerefMut)]
struct StepsTimer(Timer);

impl Default for StepsTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            TIME_BETWEEN_STEPS_WALKING,
            TimerMode::Repeating,
        ))
    }
}

fn tick_steps_timers(time: Res<Time>, mut steps_timer: ResMut<StepsTimer>) {
    steps_timer.tick(time.delta());
}

fn play_step_sounds(
    assets: Res<GameAssets>,
    mut steps_timer: ResMut<StepsTimer>,
    q_player: Query<&Player>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    let player_state = match q_player.get_single() {
        Ok(p) => p.state,
        Err(_) => return,
    };

    if !steps_timer.just_finished() {
        return;
    }

    let sound = match player_state {
        PlayerState::Walking => {
            steps_timer.set_duration(Duration::from_secs_f32(TIME_BETWEEN_STEPS_WALKING));
            Some(PlaySound {
                clip: assets.player_footstep.clone(),
                volume: 1.5,
                rand_speed_intensity: RAND_SPEED_INTENSITY,
                ..default()
            })
        }
        PlayerState::Running => {
            steps_timer.set_duration(Duration::from_secs_f32(TIME_BETWEEN_STEPS_RUNNING));
            Some(PlaySound {
                clip: assets.player_footstep.clone(),
                volume: 2.5,
                rand_speed_intensity: RAND_SPEED_INTENSITY,
                ..default()
            })
        }
        _ => None,
    };

    if let Some(s) = sound {
        ev_play_sound.send(s);
    }
}

pub struct PlayerAudioPlugin;

impl Plugin for PlayerAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (play_step_sounds, tick_steps_timers).run_if(in_state(GameState::Gaming)),
        )
        .init_resource::<StepsTimer>();
    }
}
