use std::str::FromStr;

use bevy::prelude::*;

use crate::{audio::PlaySound, npc::NpcDialogue, GameAssets, GameState};

#[derive(Event)]
pub struct PlayBlipEvent {
    dialogue: String,
}

impl PlayBlipEvent {
    pub fn new(dialogue: &str) -> Self {
        Self {
            dialogue: dialogue.to_string(),
        }
    }
}

fn character_sound(assets: &Res<GameAssets>, character: &str) -> PlaySound {
    let character = character.trim_start_matches("_");
    if character == "You" {
        return PlaySound {
            clip: assets.pai_blip_sound.clone(),
            rand_speed_intensity: 0.02,
            playback_rate: 1.2,
            volume: 0.5,
            ..default()
        };
    }

    match NpcDialogue::from_str(character) {
        Ok(r) => match r {
            NpcDialogue::Eleonore => PlaySound {
                clip: assets.eleonore_blip_sound.clone(),
                rand_speed_intensity: 0.05,
                playback_rate: 3.0,
                volume: 0.5,
                ..default()
            },
            NpcDialogue::Joanna => PlaySound {
                clip: assets.joanna_blip_sound.clone(),
                rand_speed_intensity: 0.01,
                playback_rate: 1.2,
                ..default()
            },
            NpcDialogue::Jotem => PlaySound {
                clip: assets.jotem_blip_sound.clone(),
                rand_speed_intensity: 0.01,
                playback_rate: 1.0,
                volume: 0.5,
                ..default()
            },
        },
        Err(_) => {
            if character == "???" {
                error!("You should never hardcode character name: '???' in dialogues!");
            }
            PlaySound {
                clip: assets.eleonore_blip_sound.clone(),
                ..default()
            }
        }
    }
}

fn play_blips(
    assets: Res<GameAssets>,
    mut ev_play_blip: EventReader<PlayBlipEvent>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    for ev in ev_play_blip.read() {
        ev_play_sound.send(character_sound(&assets, &ev.dialogue));
    }
}

pub struct DialogueAudioPlugin;

impl Plugin for DialogueAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (play_blips,).run_if(in_state(GameState::Gaming)))
            .add_event::<PlayBlipEvent>();
    }
}
