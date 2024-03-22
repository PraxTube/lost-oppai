use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_yarnspinner::{events::*, prelude::*};

use crate::player::chat::PlayerStoppedChat;
use crate::player::input::PlayerInput;
use crate::{GameAssets, GameState};

use super::spawn::{
    spawn_options, DialogueContent, DialogueRoot, OptionButton, OptionsBackground, OptionsNode,
};
use super::typewriter::{self, Typewriter, TypewriterFinishedEvent};
use super::DialogueViewSystemSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Event)]
struct HasSelectedOptionEvent;

#[derive(Debug, Clone, PartialEq, Default, Resource)]
pub struct OptionSelection {
    mouse_input: bool,
    current_selection: Option<usize>,
    options: Vec<DialogueOption>,
    unavailable_options: Vec<DialogueOption>,
}

impl OptionSelection {
    pub fn from_option_set<'a>(
        dialogue_options: impl IntoIterator<Item = &'a DialogueOption>,
    ) -> Self {
        let mut options: Vec<DialogueOption> = Vec::new();
        let mut unavailable_options: Vec<DialogueOption> = Vec::new();
        for option in dialogue_options {
            if option.is_available {
                options.push(option.clone());
            } else {
                unavailable_options.push(option.clone());
            }
        }

        Self {
            mouse_input: true,
            current_selection: None,
            options,
            unavailable_options,
        }
    }

    pub fn get_options(&self) -> Vec<DialogueOption> {
        self.options.clone()
    }

    pub fn get_unavailable_options(&self) -> Vec<DialogueOption> {
        self.unavailable_options.clone()
    }
}

fn create_options(
    mut commands: Commands,
    assets: Res<GameAssets>,
    option_selection: Res<OptionSelection>,
    q_children: Query<&Children>,
    mut q_options_node: Query<(Entity, &mut Style), With<OptionsNode>>,
    mut q_options_background: Query<&mut Visibility, With<OptionsBackground>>,
    mut q_root_visibility: Query<&mut Visibility, (With<DialogueRoot>, Without<OptionsBackground>)>,
) {
    let (entity, mut style) = match q_options_node.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    let mut visibility = match q_options_background.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    style.display = Display::Flex;
    *visibility = Visibility::Hidden;
    if q_children.iter_descendants(entity).next().is_none() {
        *q_root_visibility.single_mut() = Visibility::Inherited;
        spawn_options(&mut commands, &assets, &option_selection, entity);
    }
}

fn show_options(
    q_options_node: Query<Entity, With<OptionsNode>>,
    mut q_options_background: Query<&mut Visibility, With<OptionsBackground>>,
    q_children: Query<&Children>,
    mut typewriter_finished_event: EventReader<TypewriterFinishedEvent>,
) {
    if typewriter_finished_event.is_empty() {
        return;
    }
    typewriter_finished_event.clear();

    let entity = match q_options_node.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    if q_children.iter_descendants(entity).next().is_none() {
        return;
    };

    let mut visibility = match q_options_background.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    *visibility = Visibility::Inherited;
}

fn select_option(
    player_input: Res<PlayerInput>,
    typewriter: Res<Typewriter>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    mut option_selection: ResMut<OptionSelection>,
    mut q_buttons: Query<(&Interaction, &OptionButton, &Children), With<Button>>,
    mut q_text: Query<&mut Text, Without<DialogueContent>>,
    mut selected_option_event: EventWriter<HasSelectedOptionEvent>,
    mut ev_mouse_motion: EventReader<MouseMotion>,
) {
    if !typewriter.is_finished() {
        return;
    }

    if !ev_mouse_motion.is_empty() {
        ev_mouse_motion.clear();
        option_selection.mouse_input = true;
    }

    let direction = player_input.dialogue_direction;
    if direction != 0 {
        option_selection.mouse_input = false;
        match option_selection.current_selection {
            Some(r) => {
                let new_index = (r as i8 - direction)
                    .clamp(0, option_selection.options.len() as i8 - 1)
                    as usize;
                option_selection.current_selection = Some(new_index);
            }
            None => {
                if option_selection.options.len() > 0 {
                    option_selection.current_selection = Some(0);
                }
            }
        }
    }

    if option_selection.mouse_input {
        for (interaction, button, children) in &mut q_buttons {
            let color = match *interaction {
                Interaction::Pressed => {
                    selected_option_event.send(HasSelectedOptionEvent);
                    for mut dialogue_runner in &mut dialogue_runners {
                        dialogue_runner.select_option(button.0).unwrap();
                    }
                    Color::WHITE
                }
                Interaction::Hovered => {
                    option_selection.current_selection = None;
                    Color::TOMATO
                }
                _ => Color::WHITE,
            };
            let text_entity = children.iter().find(|&e| q_text.contains(*e)).unwrap();
            let mut text = q_text.get_mut(*text_entity).unwrap();
            text.sections[0].style.color = color;
        }
    } else if let Some(r) = option_selection.current_selection {
        for (i, (_, _, children)) in &mut q_buttons.iter().enumerate() {
            let color = if r == i { Color::TOMATO } else { Color::WHITE };
            let text_entity = children.iter().find(|&e| q_text.contains(*e)).unwrap();
            let mut text = q_text.get_mut(*text_entity).unwrap();
            text.sections[0].style.color = color;
        }
    }

    if !player_input.dialogue_confirm {
        return;
    }

    let selection = option_selection.current_selection;
    if let Some(index) = selection {
        let id = option_selection.options[index].id;
        selected_option_event.send(HasSelectedOptionEvent);
        for mut dialogue_runner in &mut dialogue_runners {
            dialogue_runner.select_option(id).unwrap();
        }
    }
}

fn despawn_options(
    mut commands: Commands,
    mut q_options_node: Query<Entity, With<OptionsNode>>,
    mut q_options_background: Query<&mut Visibility, With<OptionsBackground>>,
    mut q_dialogue_node_text: Query<&mut Text, With<DialogueContent>>,
    mut q_root_visibility: Query<&mut Visibility, (With<DialogueRoot>, Without<OptionsBackground>)>,
    mut ev_has_selected_option: EventReader<HasSelectedOptionEvent>,
    mut ev_dialogue_complete: EventReader<DialogueCompleteEvent>,
    mut ev_player_stopped_chat: EventReader<PlayerStoppedChat>,
) {
    if ev_has_selected_option.is_empty()
        && ev_dialogue_complete.is_empty()
        && ev_player_stopped_chat.is_empty()
    {
        return;
    }

    commands.remove_resource::<OptionSelection>();

    ev_has_selected_option.clear();
    ev_dialogue_complete.clear();
    ev_player_stopped_chat.clear();

    let entity = match q_options_node.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    let mut visibility = match q_options_background.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    commands.entity(entity).despawn_descendants();
    *visibility = Visibility::Hidden;
    *q_dialogue_node_text.single_mut() = Text::default();
    *q_root_visibility.single_mut() = Visibility::Hidden;
}

pub struct DialogueSelectionPlugin;

impl Plugin for DialogueSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                create_options.run_if(resource_added::<OptionSelection>()),
                show_options,
                select_option
                    .run_if(resource_exists::<OptionSelection>())
                    .before(typewriter::despawn),
                despawn_options,
            )
                .chain()
                .after(YarnSpinnerSystemSet)
                .in_set(DialogueViewSystemSet)
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<HasSelectedOptionEvent>();
    }
}
