mod sound;
mod spacial;

use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

#[allow(unused_imports)]
pub use sound::PlaySound;

use crate::{player::input::PlayerInput, world::ending::EndingTriggered};

const MAIN_VOLUME_DELTA: f64 = 0.05;
const FADE_IN_TIME: f32 = 3.0;
const FADE_OUT_TIME: f32 = 2.0;
const DEFAULT_VOLUME: f64 = 0.5;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_plugins((spacial::SpacialAudioPlugin, sound::GameSoundPlugin))
            .init_resource::<GameAudio>()
            .add_systems(
                Update,
                (update_main_volume, fade_in_volume, fade_out_volume),
            );
    }
}

#[derive(Resource)]
pub struct GameAudio {
    pub main_volume: f64,
}

impl Default for GameAudio {
    fn default() -> Self {
        Self {
            main_volume: DEFAULT_VOLUME,
        }
    }
}

impl GameAudio {
    pub fn update(&mut self, x: f64) {
        self.main_volume = (self.main_volume + x).clamp(0.0, 1.0);
    }
}

fn update_main_volume(player_input: Res<PlayerInput>, mut game_audio: ResMut<GameAudio>) {
    if player_input.scroll == 0.0 {
        return;
    }

    game_audio.update(-player_input.scroll as f64 * MAIN_VOLUME_DELTA);
}

fn fade_in_volume(
    time: Res<Time>,
    mut game_audio: ResMut<GameAudio>,
    mut timer: Local<Timer>,
    mut is_started: Local<bool>,
    mut is_finished: Local<bool>,
) {
    if *is_finished {
        return;
    }

    if !*is_started {
        *is_started = true;
        timer.set_duration(Duration::from_secs_f32(FADE_IN_TIME));
        timer.set_elapsed(Duration::ZERO);
    }

    timer.tick(time.delta());
    game_audio.main_volume =
        timer.elapsed().as_secs_f64() / timer.duration().as_secs_f64() * DEFAULT_VOLUME;

    if timer.just_finished() {
        game_audio.main_volume = DEFAULT_VOLUME;
        *is_finished = true;
    }
}

fn fade_out_volume(
    time: Res<Time>,
    mut game_audio: ResMut<GameAudio>,
    mut ev_ending_triggered: EventReader<EndingTriggered>,
    mut timer: Local<Timer>,
    mut is_started: Local<bool>,
    mut is_finished: Local<bool>,
    mut start_volume: Local<f64>,
) {
    if *is_finished {
        return;
    }

    if !ev_ending_triggered.is_empty() {
        ev_ending_triggered.clear();
        *is_started = true;
        *start_volume = game_audio.main_volume;
        timer.set_duration(Duration::from_secs_f32(FADE_OUT_TIME));
        timer.set_elapsed(Duration::ZERO);
    }

    if !*is_started {
        return;
    }

    timer.tick(time.delta());
    game_audio.main_volume =
        (1.0 - timer.elapsed().as_secs_f64() / timer.duration().as_secs_f64()) * *start_volume;

    if timer.just_finished() {
        game_audio.main_volume = 0.0;
        *is_finished = true;
    }
}
