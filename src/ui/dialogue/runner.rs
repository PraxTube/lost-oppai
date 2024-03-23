use bevy::prelude::*;
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::*};

use crate::{
    player::chat::{PlayerStartedChat, PlayerStoppedChat},
    GameState,
};

use super::{
    option_selection::{CreateOptions, OptionSelection},
    spawn::DialogueRoot,
    typewriter::{Typewriter, WriteDialogueText},
};

#[derive(Component)]
pub struct RunnerFlags {
    pub active: bool,
    pub dialogue: String,
    pub line: Option<LocalizedLine>,
    pub options: Option<OptionSelection>,
}

impl RunnerFlags {
    fn new(dialogue: &str) -> Self {
        Self {
            active: true,
            dialogue: dialogue.to_string(),
            line: None,
            options: None,
        }
    }
}

fn spawn_dialogue_runner(
    mut commands: Commands,
    mut typewriter: ResMut<Typewriter>,
    project: Res<YarnProject>,
    mut q_runner_flags: Query<&mut RunnerFlags>,
    mut q_dialogue: Query<&mut Visibility, With<DialogueRoot>>,
    mut ev_player_started_chat: EventReader<PlayerStartedChat>,
    mut ev_show_options: EventWriter<CreateOptions>,
    mut ev_write_dialogue_text: EventWriter<WriteDialogueText>,
) {
    let mut visibility = match q_dialogue.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    for ev in ev_player_started_chat.read() {
        *visibility = Visibility::Inherited;

        let mut cached = false;
        for mut flags in &mut q_runner_flags {
            if flags.dialogue == ev.dialogue {
                cached = true;
                flags.active = true;
                if let Some(option_selection) = &flags.options {
                    ev_show_options.send(CreateOptions(option_selection.clone()));
                    commands.insert_resource(option_selection.clone());
                }
                if let Some(line) = &flags.line {
                    typewriter.set_completed_line(line);
                    ev_write_dialogue_text.send(WriteDialogueText);
                }
            }
        }
        if !cached {
            typewriter.reset();
            let mut dialogue_runner = project.create_dialogue_runner();
            dialogue_runner.start_node(&ev.dialogue);
            commands.spawn((dialogue_runner, RunnerFlags::new(&ev.dialogue)));
        }
    }
}

fn hide_dialogue(mut q_dialogue: Query<&mut Visibility, With<DialogueRoot>>) {
    let mut visibility = match q_dialogue.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    *visibility = Visibility::Hidden;
}

fn despawn_dialogue_runner(
    mut commands: Commands,
    mut ev_dialogue_completed: EventReader<DialogueCompleteEvent>,
) {
    for ev in ev_dialogue_completed.read() {
        if let Some(r) = commands.get_entity(ev.source) {
            r.despawn_recursive();
        }
    }
}

fn deactivate_dialogue_runner(
    mut q_runner_flags: Query<&mut RunnerFlags>,
    mut ev_player_stopped_chat: EventReader<PlayerStoppedChat>,
) {
    if ev_player_stopped_chat.is_empty() {
        return;
    }
    ev_player_stopped_chat.clear();

    for mut flags in &mut q_runner_flags {
        flags.active = false;
    }
}

fn monitor_active_runners(q_runner_flags: Query<&RunnerFlags>) {
    let mut active = 0;

    for flags in &q_runner_flags {
        if flags.active {
            active += 1;
        }
    }

    if active > 1 {
        error!("There are more then 1 active flags!");
    }
}

pub struct DialogueRunnerPlugin;

impl Plugin for DialogueRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_dialogue_runner,)
                .run_if(in_state(GameState::Gaming).and_then(resource_exists::<YarnProject>())),
        )
        .add_systems(
            Update,
            (
                despawn_dialogue_runner,
                deactivate_dialogue_runner,
                hide_dialogue.run_if(
                    on_event::<DialogueCompleteEvent>().or_else(on_event::<PlayerStoppedChat>()),
                ),
                monitor_active_runners,
            ),
        );
    }
}
