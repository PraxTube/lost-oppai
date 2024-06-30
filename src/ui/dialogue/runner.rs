use bevy::prelude::*;
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::*};

use crate::{
    npc::{Npc, NpcDialogue},
    player::chat::{PlayerStartedChat, PlayerStoppedChat},
    GameState,
};

use super::{
    command::{set_type_speed_command, stop_chat_command},
    option_selection::{CreateOptions, OptionSelection},
    spawn::DialogueRoot,
    typewriter::{Typewriter, WriteDialogueText},
};

const DELAY_FRAMES_UPDATE_TARGET_NPCS: usize = 3;

#[derive(Component)]
pub struct RunnerFlags {
    pub active: bool,
    pub dialogue: NpcDialogue,
    pub line: Option<LocalizedLine>,
    pub options: Option<OptionSelection>,
    pub talked_with_target_npc: bool,
}

/// This is only fired when the dialogue runner isn't cached yet and was thus never spawned.
#[derive(Event)]
struct SpawnDialogueRunner {
    dialogue: NpcDialogue,
}

#[derive(Event)]
pub struct UpdateNpcTargets;

impl RunnerFlags {
    fn new(active: bool, dialogue: NpcDialogue) -> Self {
        Self {
            active,
            dialogue,
            line: None,
            options: None,
            talked_with_target_npc: false,
        }
    }
}

fn spawn_dialogue_runner(
    mut commands: Commands,
    mut typewriter: ResMut<Typewriter>,
    project: Res<YarnProject>,
    mut q_npcs: Query<&mut Npc>,
    mut ev_spawn_dialogue_runner: EventReader<SpawnDialogueRunner>,
    mut ev_update_npc_targets: EventWriter<UpdateNpcTargets>,
) {
    for ev in ev_spawn_dialogue_runner.read() {
        for mut npc in &mut q_npcs {
            if npc.dialogue == ev.dialogue {
                npc.was_talked_to = true;
            }
        }

        typewriter.reset();
        let mut dialogue_runner = project.create_dialogue_runner();
        dialogue_runner
            .commands_mut()
            .add_command("set_type_speed", set_type_speed_command)
            .add_command("stop_chat", stop_chat_command);

        dialogue_runner.start_node(&ev.dialogue.to_string());
        commands.spawn((dialogue_runner, RunnerFlags::new(true, ev.dialogue)));
        ev_update_npc_targets.send(UpdateNpcTargets);
    }
}

fn activate_dialogue_runner(
    mut commands: Commands,
    mut typewriter: ResMut<Typewriter>,
    mut q_runner_flags: Query<&mut RunnerFlags>,
    mut q_dialogue: Query<&mut Visibility, With<DialogueRoot>>,
    mut ev_player_started_chat: EventReader<PlayerStartedChat>,
    mut ev_show_options: EventWriter<CreateOptions>,
    mut ev_write_dialogue_text: EventWriter<WriteDialogueText>,
    mut ev_spawn_dialogue_runner: EventWriter<SpawnDialogueRunner>,
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
            ev_spawn_dialogue_runner.send(SpawnDialogueRunner {
                dialogue: ev.dialogue,
            });
        }
    }
}

fn update_npc_target(flags: &RunnerFlags, runner: &mut DialogueRunner, dialogue: NpcDialogue) {
    let variable_storage = runner.variable_storage_mut();
    let npc_target: String = match variable_storage.get("$npc_target") {
        Ok(r) => r.to_string(),
        Err(err) => {
            error!(
                "Dialogue without the variable $npc_target! In dialogue {}, {}",
                flags.dialogue, err
            );
            String::new()
        }
    };
    if npc_target.to_lowercase() == dialogue.to_string().to_lowercase() {
        if let Err(err) = variable_storage.set("$talked_with_target_npc".to_string(), (true).into())
        {
            error!(
                "Error while trying to get the variable $talked_with_target_npc! In dialogue {}, {}",
                flags.dialogue, err
            );
        }
    }
}

fn update_npc_targets(
    q_npcs: Query<&Npc>,
    mut q_dialogue_runners: Query<(&mut DialogueRunner, &RunnerFlags)>,
    mut ev_update_npc_targets: EventReader<UpdateNpcTargets>,
    mut started: Local<bool>,
    mut frames: Local<usize>,
) {
    for _ev in ev_update_npc_targets.read() {
        *started = true;
    }

    if !*started {
        return;
    }

    if *frames < DELAY_FRAMES_UPDATE_TARGET_NPCS {
        *frames += 1;
        return;
    }

    *started = false;
    *frames = 0;
    for (mut runner, flags) in &mut q_dialogue_runners {
        for npc in &q_npcs {
            if npc.was_talked_to {
                update_npc_target(flags, &mut runner, npc.dialogue);
            }
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
            (activate_dialogue_runner, spawn_dialogue_runner)
                .chain()
                .run_if(in_state(GameState::Gaming).and_then(resource_exists::<YarnProject>())),
        )
        .add_event::<SpawnDialogueRunner>()
        .add_event::<UpdateNpcTargets>()
        .add_systems(
            Update,
            (
                despawn_dialogue_runner,
                deactivate_dialogue_runner,
                hide_dialogue.run_if(
                    on_event::<DialogueCompleteEvent>().or_else(on_event::<PlayerStoppedChat>()),
                ),
                monitor_active_runners,
                update_npc_targets,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
