use rand::{thread_rng, Rng};

use bevy::{prelude::*, utils::HashSet};
use bevy_kira_audio::prelude::{AudioSource, *};

use super::GameAudio;

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
        audio_command
            .with_volume(ev.volume * volume_offset * game_audio.main_volume)
            .with_playback_rate(ev.playback_rate + speed_offset);

        if ev.repeat {
            audio_command.looped();
        }
        if ev.reverse {
            audio_command.reverse();
        }

        let audio_instance = audio_command.handle();

        if let Some(parent) = ev.parent {
            let audio_emitter = commands
                .spawn((
                    TransformBundle::default(),
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

pub struct GameSoundPlugin;

impl Plugin for GameSoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaySound>()
            .init_resource::<GameAudio>()
            .add_systems(Update, (play_sounds,));
    }
}
