use std::str::FromStr;

use bevy::prelude::*;

use crate::{
    audio::PlaySound,
    npc::NpcDialogue,
    ui::main_menu::{ButtonAction, MainMenuButtonPressed},
    GameAssets,
};

#[derive(Resource)]
struct BlipManager {
    play_blips: bool,
    volume_multiplier: f64,
}

impl Default for BlipManager {
    fn default() -> Self {
        Self {
            play_blips: true,
            volume_multiplier: 1.0,
        }
    }
}

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
    let character = character.trim_start_matches('_');
    // Narrator, i.e. no character name on screen.
    if character.is_empty() {
        return PlaySound {
            clip: assets.narrator_blip_sound.clone(),
            rand_speed_intensity: 0.02,
            playback_rate: 1.0,
            volume: 0.2,
            ..default()
        };
    }

    if character == "You" {
        return PlaySound {
            clip: assets.npc_blip_sound.clone(),
            rand_speed_intensity: 0.02,
            playback_rate: 2.7,
            volume: 0.5,
            ..default()
        };
    }

    match NpcDialogue::from_str(character) {
        Ok(r) => match r {
            NpcDialogue::Eleonore => PlaySound {
                clip: assets.npc_blip_sound.clone(),
                rand_speed_intensity: 0.03,
                playback_rate: 2.9,
                volume: 0.5,
                ..default()
            },
            NpcDialogue::Jotem => PlaySound {
                clip: assets.npc_blip_sound.clone(),
                rand_speed_intensity: 0.01,
                playback_rate: 1.0,
                volume: 0.5,
                ..default()
            },
            NpcDialogue::Isabelle => PlaySound {
                clip: assets.npc_blip_sound.clone(),
                rand_speed_intensity: 0.04,
                playback_rate: 3.5,
                volume: 0.45,
                ..default()
            },
            NpcDialogue::Ionas => PlaySound {
                clip: assets.npc_blip_sound.clone(),
                rand_speed_intensity: 0.012,
                playback_rate: 2.0,
                volume: 0.5,
                ..default()
            },
            NpcDialogue::Antonius => PlaySound {
                clip: assets.npc_blip_sound.clone(),
                rand_speed_intensity: 0.02,
                playback_rate: 2.2,
                volume: 0.5,
                ..default()
            },
            NpcDialogue::Sven => PlaySound {
                clip: assets.npc_blip_sound.clone(),
                rand_speed_intensity: 0.015,
                playback_rate: 1.3,
                volume: 0.5,
                ..default()
            },
            NpcDialogue::Joanna => PlaySound {
                clip: assets.npc_blip_sound.clone(),
                rand_speed_intensity: 0.01,
                playback_rate: 2.5,
                volume: 0.5,
                ..default()
            },
            NpcDialogue::Dorothea => PlaySound {
                clip: assets.npc_blip_sound.clone(),
                rand_speed_intensity: 0.02,
                playback_rate: 2.0,
                volume: 0.5,
                ..default()
            },
            NpcDialogue::IonasAndAntonius => PlaySound {
                clip: assets.npc_blip_sound.clone(),
                ..default()
            },
            NpcDialogue::Paladins => PlaySound {
                clip: assets.npc_blip_sound.clone(),
                ..default()
            },
        },
        Err(_) => {
            if character == "???" {
                error!("You should never hardcode character name: '???' in dialogues!");
            }
            PlaySound {
                clip: assets.npc_blip_sound.clone(),
                ..default()
            }
        }
    }
}

fn play_blips(
    assets: Res<GameAssets>,
    blip_manager: Res<BlipManager>,
    mut ev_play_blip: EventReader<PlayBlipEvent>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    if !blip_manager.play_blips {
        return;
    }

    for ev in ev_play_blip.read() {
        let mut play_sound = character_sound(&assets, &ev.dialogue);
        play_sound.volume *= blip_manager.volume_multiplier;
        ev_play_sound.send(play_sound);
    }
}

fn toggle_play_blips(
    mut blip_manager: ResMut<BlipManager>,
    mut ev_main_menu_button_pressed: EventReader<MainMenuButtonPressed>,
) {
    for ev in ev_main_menu_button_pressed.read() {
        if ev.0 == ButtonAction::ToggleBlips {
            blip_manager.play_blips = !blip_manager.play_blips;
        }
    }
}

fn update_volume_multiplier(
    mut blip_manager: ResMut<BlipManager>,
    mut ev_main_menu_button_pressed: EventReader<MainMenuButtonPressed>,
) {
    for ev in ev_main_menu_button_pressed.read() {
        let volume_multiplier = match ev.0 {
            ButtonAction::Normal => 1.0,
            ButtonAction::Quick => 0.6,
            ButtonAction::Fast => 0.4,
            ButtonAction::Instant => 1.0,
            _ => continue,
        };
        blip_manager.volume_multiplier = volume_multiplier;
    }
}

pub struct DialogueAudioPlugin;

impl Plugin for DialogueAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BlipManager>()
            .add_event::<PlayBlipEvent>()
            .add_systems(
                Update,
                (
                    play_blips.run_if(resource_exists::<GameAssets>),
                    toggle_play_blips,
                    update_volume_multiplier,
                ),
            );
    }
}
