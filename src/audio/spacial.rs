use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::GameAudio;

const MAX_DISTANCE: f64 = 250.0;

#[derive(Component)]
pub struct SpacialSound {
    volume: f64,
}

impl SpacialSound {
    pub fn new(volume: f64) -> Self {
        Self { volume }
    }
}

fn update(
    game_audio: &Res<GameAudio>,
    receiver_transform: &GlobalTransform,
    emitters: &Query<(&GlobalTransform, &AudioEmitter, &SpacialSound)>,
    audio_instances: &mut Assets<AudioInstance>,
) {
    for (emitter_transform, emitter, sound) in emitters {
        let distance = (emitter_transform.translation() - receiver_transform.translation())
            .truncate()
            .length_squared();
        let multiplier = (1.0 - distance as f64 / MAX_DISTANCE.powi(2)).clamp(0.0, 1.0);
        let volume: f64 = sound.volume * multiplier.powi(2) * game_audio.main_volume;

        for instance in emitter.instances.iter() {
            if let Some(instance) = audio_instances.get_mut(instance) {
                instance.set_volume(volume, AudioTween::default());
            }
        }
    }
}

fn update_volumes(
    game_audio: Res<GameAudio>,
    receiver: Query<&GlobalTransform, With<AudioReceiver>>,
    emitters: Query<(&GlobalTransform, &AudioEmitter, &SpacialSound)>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Ok(receiver_transform) = receiver.get_single() {
        update(
            &game_audio,
            receiver_transform,
            &emitters,
            &mut audio_instances,
        );
    }
}

fn cleanup_stopped_spacial_instances(
    mut emitters: Query<&mut AudioEmitter>,
    instances: ResMut<Assets<AudioInstance>>,
) {
    for mut emitter in emitters.iter_mut() {
        let handles = &mut emitter.instances;

        handles.retain(|handle| {
            if let Some(instance) = instances.get(handle) {
                instance.state() != PlaybackState::Stopped
            } else {
                true
            }
        });
    }
}

pub struct SpacialAudioPlugin;

impl Plugin for SpacialAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_volumes, cleanup_stopped_spacial_instances));
    }
}
