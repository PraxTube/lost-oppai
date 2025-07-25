use bevy::prelude::*;
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::*};

use crate::{
    npc::{Npc, NpcDialogue},
    player::chat::{PlayerStartedChat, PlayerStoppedChat},
    world::ending::EndingTriggered,
    GameState,
};

use super::{
    command::{stop_chat_command, target_npc_mentioned_command, trigger_ending_command},
    option_selection::{CreateOptions, OptionSelection},
    spawn::{DialogueContent, DialogueRoot},
    typewriter::{Typewriter, WriteDialogueText},
};

const DELAY_FRAMES_UPDATE_TARGET_NPCS: usize = 3;

#[derive(Component)]
pub struct RunnerFlags {
    pub active: bool,
    pub dialogue: NpcDialogue,
    pub line: Option<LocalizedLine>,
    pub options: Option<OptionSelection>,
}

/// This is only fired when the dialogue runner isn't cached yet and was thus never spawned.
#[derive(Event)]
struct SpawnDialogueRunner {
    dialogue: NpcDialogue,
}

#[derive(Event)]
pub struct UpdateTargetNpcs;

impl RunnerFlags {
    fn new(dialogue: NpcDialogue) -> Self {
        Self {
            active: true,
            dialogue,
            line: None,
            options: None,
        }
    }
}

fn spawn_dialogue_runner(
    mut commands: Commands,
    mut typewriter: ResMut<Typewriter>,
    project: Res<YarnProject>,
    mut q_npcs: Query<&mut Npc>,
    mut ev_spawn_dialogue_runner: EventReader<SpawnDialogueRunner>,
    mut ev_update_target_npcs: EventWriter<UpdateTargetNpcs>,
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
            .add_command("stop_chat", stop_chat_command)
            .add_command("target_npc_mentioned", target_npc_mentioned_command)
            .add_command("trigger_ending", trigger_ending_command);

        dialogue_runner.start_node(ev.dialogue.to_string());
        commands.spawn((dialogue_runner, RunnerFlags::new(ev.dialogue)));
        ev_update_target_npcs.send(UpdateTargetNpcs);
    }
}

fn activate_dialogue_runner(
    mut commands: Commands,
    mut typewriter: ResMut<Typewriter>,
    mut q_runner_flags: Query<&mut RunnerFlags>,
    mut q_dialogue: Query<&mut Visibility, With<DialogueRoot>>,
    mut q_dialogue_content: Query<&mut Text, With<DialogueContent>>,
    mut ev_player_started_chat: EventReader<PlayerStartedChat>,
    mut ev_create_options: EventWriter<CreateOptions>,
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
                    ev_create_options.send(CreateOptions(option_selection.clone()));
                    commands.insert_resource(option_selection.clone());
                }
                if let Some(line) = &flags.line {
                    typewriter.set_completed_line(line);
                    ev_write_dialogue_text.send(WriteDialogueText);
                }
            }
        }
        if !cached {
            if let Ok(mut dialogue_content) = q_dialogue_content.get_single_mut() {
                *dialogue_content = Text::default();
            }
            ev_spawn_dialogue_runner.send(SpawnDialogueRunner {
                dialogue: ev.dialogue,
            });
        }
    }
}

fn update_target_npc(flags: &RunnerFlags, runner: &mut DialogueRunner, dialogue: NpcDialogue) {
    let variable_storage = runner.variable_storage_mut();
    let target_npc: String = match variable_storage.get("$target_npc") {
        Ok(r) => r.to_string(),
        Err(err) => {
            error!(
                "Dialogue without the variable $target_npc! In dialogue {}, {}",
                flags.dialogue, err
            );
            String::new()
        }
    };
    if target_npc == dialogue.to_string() {
        if let Err(err) = variable_storage.set("$talked_with_target_npc".to_string(), (true).into())
        {
            error!("{}, {}", flags.dialogue, err);
        }
    }
}

fn update_mentioned_by(
    flags: &RunnerFlags,
    runner: &mut DialogueRunner,
    mentioned_by: &NpcDialogue,
) {
    let variable_storage = runner.variable_storage_mut();
    let variable = format!("$mentioned_by_{mentioned_by}");
    if !variable_storage.contains(&variable) {
        error!("Npc {}, does not contain var {}", flags.dialogue, variable);
        return;
    };
    if let Err(err) = variable_storage.set(variable, (true).into()) {
        error!("{}, {}", flags.dialogue, err);
    }
}

fn update_target_npcs(
    q_npcs: Query<&Npc>,
    mut q_dialogue_runners: Query<(&mut DialogueRunner, &RunnerFlags)>,
    mut ev_update_target_npcs: EventReader<UpdateTargetNpcs>,
    mut started: Local<bool>,
    mut frames: Local<usize>,
) {
    for _ev in ev_update_target_npcs.read() {
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
                update_target_npc(flags, &mut runner, npc.dialogue);
            }

            if flags.dialogue == npc.dialogue {
                for mentioned_by in &npc.was_mentioned_by {
                    update_mentioned_by(flags, &mut runner, mentioned_by);
                }
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

fn deactivate_dialogue_runner(mut q_runner_flags: Query<&mut RunnerFlags>) {
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
        error!("There are more than 1 active flags!");
    }
}

pub struct DialogueRunnerPlugin;

impl Plugin for DialogueRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnDialogueRunner>()
            .add_event::<UpdateTargetNpcs>()
            .add_systems(
                Update,
                (activate_dialogue_runner, spawn_dialogue_runner)
                    .chain()
                    .run_if(in_state(GameState::Gaming).and_then(resource_exists::<YarnProject>)),
            )
            .add_systems(
                Update,
                (
                    despawn_dialogue_runner,
                    deactivate_dialogue_runner.run_if(on_event::<PlayerStoppedChat>()),
                    monitor_active_runners,
                    update_target_npcs,
                ),
            )
            .add_systems(
                Update,
                hide_dialogue.run_if(on_event::<DialogueCompleteEvent>().or_else(
                    on_event::<PlayerStoppedChat>().or_else(on_event::<EndingTriggered>()),
                )),
            );
    }
}
