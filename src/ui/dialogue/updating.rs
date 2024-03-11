use bevy::prelude::*;
use bevy_yarnspinner::{events::*, prelude::*};

use crate::player::input::PlayerInput;

use super::option_selection::OptionSelection;
use super::spawn::{DialogueContinueNode, DialogueNameNode, DialogueRoot};
use super::typewriter::{self, Typewriter};
use super::DialogueViewSystemSet;

/// Signals that a speaker has changed.
/// A speaker starts speaking when a new line is presented with a [`PresentLineEvent`] which has a character name.
/// A speaker stops speaking when the line is fully displayed on the screen, which happens over the course of a few seconds
#[derive(Debug, Eq, PartialEq, Hash, Reflect, Event)]
#[reflect(Debug, PartialEq, Hash)]
#[non_exhaustive]
pub struct SpeakerChangeEvent {
    /// The name of the character who is or was speaking.
    pub character_name: String,
    /// If `true`, the character just started speaking. Otherwise, they just stopped.
    pub speaking: bool,
}

fn show_dialog(mut visibility: Query<&mut Visibility, With<DialogueRoot>>) {
    *visibility.single_mut() = Visibility::Inherited;
}

fn hide_dialog(
    mut root_visibility: Query<&mut Visibility, With<DialogueRoot>>,
    mut dialogue_complete_events: EventReader<DialogueCompleteEvent>,
) {
    if !dialogue_complete_events.is_empty() {
        *root_visibility.single_mut() = Visibility::Hidden;
        dialogue_complete_events.clear();
    }
}

fn present_line(
    mut line_events: EventReader<PresentLineEvent>,
    mut speaker_change_events: EventWriter<SpeakerChangeEvent>,
    mut typewriter: ResMut<Typewriter>,
    mut name_node: Query<&mut Text, With<DialogueNameNode>>,
) {
    for event in line_events.read() {
        let name = if let Some(name) = event.line.character_name() {
            speaker_change_events.send(SpeakerChangeEvent {
                character_name: name.to_string(),
                speaking: true,
            });
            name.to_string()
        } else {
            String::new()
        };
        name_node.single_mut().sections[0].value = name;
        typewriter.set_line(&event.line);
    }
}

fn present_options(mut commands: Commands, mut events: EventReader<PresentOptionsEvent>) {
    for event in events.read() {
        let option_selection = OptionSelection::from_option_set(&event.options);
        commands.insert_resource(option_selection);
    }
}

fn continue_dialogue(
    input: Res<PlayerInput>,
    mut typewriter: ResMut<Typewriter>,
    option_selection: Option<Res<OptionSelection>>,
    mut q_dialogue_runners: Query<&mut DialogueRunner>,
    mut q_root_visibility: Query<&mut Visibility, With<DialogueRoot>>,
    mut q_continue_visibility: Query<
        &mut Visibility,
        (With<DialogueContinueNode>, Without<DialogueRoot>),
    >,
) {
    if input.dialogue_fast_forward && !typewriter.is_finished() {
        typewriter.fast_forward();
        return;
    }

    if (input.dialogue_fast_forward || typewriter.last_before_options) && option_selection.is_none()
    {
        for mut dialogue_runner in q_dialogue_runners.iter_mut() {
            if !dialogue_runner.is_waiting_for_option_selection() && dialogue_runner.is_running() {
                dialogue_runner.continue_in_next_update();
                *q_root_visibility.single_mut() = Visibility::Hidden;
                *q_continue_visibility.single_mut() = Visibility::Hidden;
            }
        }
    }
}

pub struct DialogueUpdatingPlugin;

impl Plugin for DialogueUpdatingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                hide_dialog,
                show_dialog.run_if(on_event::<DialogueStartEvent>()),
                present_line.run_if(
                    resource_exists::<Typewriter>().and_then(on_event::<PresentLineEvent>()),
                ),
                present_options.run_if(on_event::<PresentOptionsEvent>()),
                continue_dialogue.run_if(resource_exists::<Typewriter>()),
            )
                .chain()
                .after(YarnSpinnerSystemSet)
                .after(typewriter::spawn)
                .in_set(DialogueViewSystemSet),
        )
        .add_event::<SpeakerChangeEvent>()
        .register_type::<SpeakerChangeEvent>();
    }
}
