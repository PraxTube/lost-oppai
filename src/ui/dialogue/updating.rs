use std::str::FromStr;

use bevy::prelude::*;
use bevy_yarnspinner::{events::*, prelude::*};

use crate::npc::NpcDialogue;
use crate::player::chat::PlayerStoppedChat;
use crate::player::input::PlayerInput;
use crate::GameAssets;

use super::option_selection::OptionSelection;
use super::runner::RunnerFlags;
use super::spawn::{DialogueCharacterIcon, DialogueContinueNode, DialogueNameNode};
use super::typewriter::{Typewriter, TypewriterFinished, WriteDialogueText};
use super::DialogueViewSystemSet;

fn convert_name(name: &str) -> String {
    if name.starts_with('_') {
        return "???".to_string();
    }
    name.to_string()
}

/// Return an Option so that you only set the texture when there is a proper NPC.
/// If there is a frame delay (due to events or similar), then we will simply
/// display the previous NPC for couple of frames. That's okay.
fn character_icon(assets: &Res<GameAssets>, name: &str) -> Option<Handle<Image>> {
    let name = name.trim_start_matches('_');
    if name == "You" {
        return Some(assets.pai_icon.clone());
    }

    match NpcDialogue::from_str(name) {
        Ok(r) => match r {
            NpcDialogue::Eleonore => Some(assets.eleonore_icon.clone()),
            NpcDialogue::Jotem => Some(assets.jotem_icon.clone()),
            NpcDialogue::Isabelle => Some(assets.isabelle_icon.clone()),
            NpcDialogue::Ionas => Some(assets.ionas_icon.clone()),
            NpcDialogue::Antonius => Some(assets.antonius_icon.clone()),
            NpcDialogue::IonasAndAntonius => {
                error!("should never happen, you have used '$name' in antonius and ionas dialogue");
                None
            }
        },
        Err(_) => None,
    }
}

fn present_line(
    assets: Res<GameAssets>,
    mut typewriter: ResMut<Typewriter>,
    mut q_character_icon: Query<&mut UiImage, With<DialogueCharacterIcon>>,
    mut q_name_text: Query<&mut Text, With<DialogueNameNode>>,
    mut q_runner_flags: Query<&mut RunnerFlags>,
    mut ev_present_line: EventReader<PresentLineEvent>,
) {
    let mut character_icon_image = match q_character_icon.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    let mut name_text = match q_name_text.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    for event in ev_present_line.read() {
        let raw_name = event.line.character_name().unwrap_or_default();
        if let Some(texture) = character_icon(&assets, raw_name) {
            character_icon_image.texture = texture;
        }
        name_text.sections[0].value = convert_name(raw_name);

        typewriter.set_line(&event.line);
        for mut flags in &mut q_runner_flags {
            if flags.active {
                flags.line = Some(event.line.clone());
            }
        }
    }
}

fn present_options(
    mut commands: Commands,
    mut q_runner_flags: Query<&mut RunnerFlags>,
    mut events: EventReader<PresentOptionsEvent>,
) {
    for event in events.read() {
        let option_selection = OptionSelection::from_option_set(&event.options);
        commands.insert_resource(option_selection.clone());

        for mut flags in &mut q_runner_flags {
            if flags.active {
                flags.options = Some(option_selection.clone());
            }
        }
    }
}

fn continue_dialogue(
    input: Res<PlayerInput>,
    typewriter: Res<Typewriter>,
    option_selection: Option<Res<OptionSelection>>,
    mut q_dialogue_runners: Query<(&mut DialogueRunner, &RunnerFlags)>,
    mut q_continue_visibility: Query<&mut Visibility, With<DialogueContinueNode>>,
) {
    if input.dialogue_continue && !typewriter.is_finished() {
        return;
    }

    if option_selection.is_some() {
        return;
    }
    if !input.dialogue_continue && !typewriter.last_before_options {
        return;
    }

    for (mut dialogue_runner, flags) in &mut q_dialogue_runners {
        if !flags.active {
            continue;
        }

        if !dialogue_runner.is_waiting_for_option_selection() && dialogue_runner.is_running() {
            dialogue_runner.continue_in_next_update();
            let mut visibility = match q_continue_visibility.get_single_mut() {
                Ok(r) => r,
                Err(_) => continue,
            };
            *visibility = Visibility::Hidden;
        }
    }
}

fn update_displayed_character(
    assets: Res<GameAssets>,
    typewriter: Res<Typewriter>,
    mut q_character_icon: Query<&mut UiImage, With<DialogueCharacterIcon>>,
    mut q_name_text: Query<&mut Text, With<DialogueNameNode>>,
) {
    let mut character_icon_image = match q_character_icon.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    let mut name_text = match q_name_text.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let raw_name = &typewriter.character_name.clone().unwrap_or_default();
    if let Some(texture) = character_icon(&assets, raw_name) {
        character_icon_image.texture = texture;
    }
    name_text.sections[0].value = convert_name(raw_name);
}

fn show_continue_node(
    typewriter: Res<Typewriter>,
    mut q_visibility: Query<&mut Visibility, With<DialogueContinueNode>>,
) {
    let mut visibility = match q_visibility.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let vis = if typewriter.last_before_options {
        Visibility::Hidden
    } else {
        Visibility::Inherited
    };
    *visibility = vis;
}

fn hide_continue_node(
    mut q_continue_visibility: Query<&mut Visibility, With<DialogueContinueNode>>,
) {
    let mut visibility = match q_continue_visibility.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    *visibility = Visibility::Hidden;
}

pub struct DialogueUpdatingPlugin;

impl Plugin for DialogueUpdatingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                present_line,
                present_options.run_if(on_event::<PresentOptionsEvent>()),
                continue_dialogue,
                update_displayed_character.run_if(on_event::<WriteDialogueText>()),
                show_continue_node.run_if(
                    on_event::<TypewriterFinished>().or_else(on_event::<WriteDialogueText>()),
                ),
                hide_continue_node.run_if(
                    on_event::<DialogueCompleteEvent>().or_else(on_event::<PlayerStoppedChat>()),
                ),
            )
                .chain()
                .run_if(resource_exists::<GameAssets>)
                .after(YarnSpinnerSystemSet)
                .in_set(DialogueViewSystemSet),
        );
    }
}
