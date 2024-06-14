use rand::{thread_rng, Rng};

use bevy::{prelude::*, utils::HashSet};
use bevy_kira_audio::prelude::{AudioSource, *};

use crate::GameState;

use super::{spacial::SpacialSound, GameAudio};

#[derive(Resource, Deref, DerefMut, Default)]
struct RepeatingSounds(Vec<(f64, Handle<AudioInstance>)>);

#[derive(Event)]
pub struct PlaySound {
    pub clip: Handle<AudioSource>,
    pub volume: f64,
    /// Playback rate, default is 1.0.
    pub playback_rate: f64,
    /// Playback offset intensity. This will add a random offset
    /// to the playback rate by this intensity.
    /// Useful for pitch shifting a sound by a certain range.
    pub rand_speed_intensity: f64,
    pub repeat: bool,
    pub reverse: bool,
    /// If you want to have spacial audio, you must give a parent entity.
    pub parent: Option<Entity>,
}

impl Default for PlaySound {
    fn default() -> Self {
        Self {
            clip: Handle::default(),
            volume: 1.0,
            playback_rate: 1.0,
            rand_speed_intensity: 0.0,
            repeat: false,
            reverse: false,
            parent: None,
        }
    }
}

fn play_sounds(
    mut commands: Commands,
    audio: Res<Audio>,
    game_audio: Res<GameAudio>,
    mut repeating_sounds: ResMut<RepeatingSounds>,
    mut ev_play_sound: EventReader<PlaySound>,
) {
    let mut rng = thread_rng();
    let mut added_sounds: HashSet<Handle<AudioSource>> = HashSet::new();

    for ev in ev_play_sound.read() {
        if added_sounds.contains(&ev.clip) {
            continue;
        }
        added_sounds.insert(ev.clip.clone());

        let speed_offset = if ev.rand_speed_intensity == 0.0 {
            0.0
        } else {
            rng.gen_range(-1.0..1.0) * ev.rand_speed_intensity
        };
        let volume_offset = if ev.parent.is_some() { 0.0 } else { 1.0 };

        let mut audio_command = audio.play(ev.clip.clone());
        let sound_volume = ev.volume * volume_offset;
        audio_command
            .with_volume(sound_volume * game_audio.main_volume)
            .with_playback_rate(ev.playback_rate + speed_offset);

        let audio_instance = audio_command.handle();

        if ev.repeat {
            audio_command.looped();
            repeating_sounds.push((sound_volume, audio_instance.clone()));
        }
        if ev.reverse {
            audio_command.reverse();
        }

        if let Some(parent) = ev.parent {
            let audio_emitter = commands
                .spawn((
                    TransformBundle::default(),
                    SpacialSound::new(ev.volume),
                    AudioEmitter {
                        instances: vec![audio_instance],
                    },
                ))
                .id();

            match commands.get_entity(parent) {
                Some(mut r) => {
                    r.push_children(&[audio_emitter]);
                }
                None => {
                    warn!("audio parent does not exist");
                }
            };
        };
    }
}

fn update_repeating_sounds(
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    game_audio: Res<GameAudio>,
    mut repeating_sounds: ResMut<RepeatingSounds>,
) {
    let mut invalid_indices = vec![];
    for (index, (volume, instance)) in repeating_sounds.iter().enumerate() {
        match audio_instances.get_mut(instance) {
            Some(r) => {
                r.set_volume(volume * game_audio.main_volume, AudioTween::default());
            }
            None => {
                invalid_indices.push(index);
            }
        }
    }

    for index in invalid_indices.iter().rev() {
        repeating_sounds.remove(*index);
    }
}

pub struct GameSoundPlugin;

impl Plugin for GameSoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaySound>()
            .init_resource::<GameAudio>()
            .init_resource::<RepeatingSounds>()
            .add_systems(
                Update,
                (
                    update_repeating_sounds
                        .run_if(resource_changed::<GameAudio>())
                        .before(play_sounds),
                    play_sounds,
                )
                    .run_if(in_state(GameState::Gaming)),
            );
    }
}
