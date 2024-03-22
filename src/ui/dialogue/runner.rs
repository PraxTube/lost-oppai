use bevy::prelude::*;
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::*};

use crate::{
    player::chat::{PlayerStartedChat, PlayerStoppedChat},
    GameState,
};

use super::{option_selection::OptionSelection, spawn::Dialogue};

#[derive(Component)]
pub struct RunnerFlags {
    pub active: bool,
    pub dialogue: String,
    pub options: Option<OptionSelection>,
}

impl RunnerFlags {
    fn new(dialogue: &str) -> Self {
        Self {
            active: true,
            dialogue: dialogue.to_string(),
            options: None,
        }
    }
}

fn spawn_dialogue_runner(
    mut commands: Commands,
    project: Res<YarnProject>,
    mut q_runner_flags: Query<&mut RunnerFlags>,
    mut q_dialogue: Query<&mut Visibility, With<Dialogue>>,
    mut ev_player_started_chat: EventReader<PlayerStartedChat>,
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
            }
        }
        if !cached {
            let mut dialogue_runner = project.create_dialogue_runner();
            dialogue_runner.start_node(&ev.dialogue);
            commands.spawn((dialogue_runner, RunnerFlags::new(&ev.dialogue)));
        }
    }
}

fn deactivate_dialogue_runner(
    mut commands: Commands,
    mut q_dialogue: Query<&mut Visibility, With<Dialogue>>,
    mut q_runner_flags: Query<&mut RunnerFlags>,
    mut ev_player_stopped_chat: EventReader<PlayerStoppedChat>,
    mut ev_dialogue_completed: EventReader<DialogueCompleteEvent>,
) {
    let mut visibility = match q_dialogue.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if ev_player_stopped_chat.is_empty() {
        for ev in ev_dialogue_completed.read() {
            if let Some(r) = commands.get_entity(ev.source) {
                r.despawn_recursive();
            }
        }
        return;
    }
    ev_player_stopped_chat.clear();

    *visibility = Visibility::Hidden;
    for mut flags in &mut q_runner_flags {
        flags.active = false;
    }
}

pub struct DialogueRunnerPlugin;

impl Plugin for DialogueRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_dialogue_runner, deactivate_dialogue_runner)
                .run_if(in_state(GameState::Gaming).and_then(resource_exists::<YarnProject>())),
        );
    }
}
