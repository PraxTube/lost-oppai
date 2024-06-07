use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use noisy_bevy::simplex_noise_2d_seeded;

use crate::{
    audio::{GameAudio, PlaySound},
    GameAssets, GameState,
};

use super::{Player, PlayerState};

const RAND_SPEED_INTENSITY: f64 = 0.2;
const TIME_BETWEEN_STEPS_WALKING: f32 = 0.5;
const TIME_BETWEEN_STEPS_RUNNING: f32 = 0.4;
const WALK_VOLUME: f64 = 1.5;
const RUN_VOLUME: f64 = 2.0;

const BIRD_MAX_VOLUME: f64 = 1.0;
const BIRD_MIN_VOLUME: f32 = 0.1;
const NOISE_ZOOM: f32 = 0.02;
const SEED: f32 = 64.0;

#[derive(Resource, Deref, DerefMut)]
struct StepsTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
struct BirdSound {
    handle: Handle<AudioInstance>,
}

impl Default for StepsTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            TIME_BETWEEN_STEPS_WALKING,
            TimerMode::Repeating,
        ))
    }
}

fn spawn_bird_sound(mut commands: Commands, assets: Res<GameAssets>, audio: Res<Audio>) {
    let handle = audio
        .play(assets.bird_sounds.clone())
        .with_volume(0.0)
        .looped()
        .handle();
    commands.insert_resource(BirdSound { handle });
}

fn update_bird_sound(
    time: Res<Time>,
    game_audio: Res<GameAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    bird_sound: Res<BirdSound>,
) {
    let noise = simplex_noise_2d_seeded(Vec2::ONE * time.elapsed_seconds() * NOISE_ZOOM, SEED)
        .max(BIRD_MIN_VOLUME);
    let volume = noise as f64 * game_audio.main_volume * BIRD_MAX_VOLUME;
    if let Some(instance) = audio_instances.get_mut(bird_sound.handle.clone()) {
        instance.set_volume(volume, AudioTween::default());
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
                volume: WALK_VOLUME,
                rand_speed_intensity: RAND_SPEED_INTENSITY,
                ..default()
            })
        }
        PlayerState::Running => {
            steps_timer.set_duration(Duration::from_secs_f32(TIME_BETWEEN_STEPS_RUNNING));
            Some(PlaySound {
                clip: assets.player_footstep.clone(),
                volume: RUN_VOLUME,
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
            (update_bird_sound, play_step_sounds, tick_steps_timers)
                .run_if(in_state(GameState::Gaming)),
        )
        .init_resource::<StepsTimer>()
        .add_systems(OnExit(GameState::AssetLoading), (spawn_bird_sound,));
    }
}
