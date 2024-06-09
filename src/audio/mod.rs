// mod bgm;
mod sound;
mod spacial;

use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

#[allow(unused_imports)]
pub use sound::PlaySound;

use crate::player::input::PlayerInput;

const MAIN_VOLUME_DELTA: f64 = 0.05;
const FADE_IN_TIME: f32 = 3.0;
const DEFAULT_VOLUME: f64 = 0.5;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_plugins((
                // bgm::BgmPlugin,
                spacial::SpacialAudioPlugin,
                sound::GameSoundPlugin,
            ))
            .init_resource::<GameAudio>()
            .add_systems(Update, (update_main_volume, fade_in_volume));
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
