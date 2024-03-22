use bevy::prelude::*;
use bevy_yarnspinner::{events::*, prelude::*};

use crate::player::input::PlayerInput;

use super::option_selection::OptionSelection;
use super::runner::RunnerFlags;
use super::spawn::{DialogueContinueNode, DialogueNameNode};
use super::typewriter::{self, Typewriter};
use super::DialogueViewSystemSet;

fn present_line(
    mut typewriter: ResMut<Typewriter>,
    mut q_name_text: Query<&mut Text, With<DialogueNameNode>>,
    mut ev_present_line: EventReader<PresentLineEvent>,
) {
    for event in ev_present_line.read() {
        let name = if let Some(name) = event.line.character_name() {
            name.to_string()
        } else {
            String::new()
        };
        q_name_text.single_mut().sections[0].value = name;
        typewriter.set_line(&event.line);
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
    mut typewriter: ResMut<Typewriter>,
    option_selection: Option<Res<OptionSelection>>,
    mut q_dialogue_runners: Query<&mut DialogueRunner>,
    mut q_continue_visibility: Query<&mut Visibility, With<DialogueContinueNode>>,
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
        );
    }
}
