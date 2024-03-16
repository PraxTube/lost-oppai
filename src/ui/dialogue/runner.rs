use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;

use crate::{
    player::chat::{PlayerStartedChat, PlayerStoppedChat},
    GameState,
};

use super::spawn::Dialogue;

fn spawn_dialogue_runner(
    mut commands: Commands,
    project: Res<YarnProject>,
    mut q_dialogue: Query<&mut Visibility, With<Dialogue>>,
    mut ev_player_started_chat: EventReader<PlayerStartedChat>,
) {
    let mut visibility = match q_dialogue.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    for ev in ev_player_started_chat.read() {
        let mut dialogue_runner = project.create_dialogue_runner();
        dialogue_runner.start_node(&ev.dialogue);
        *visibility = Visibility::Inherited;
        commands.spawn(dialogue_runner);
    }
}

fn despawn_dialogue_runner(
    mut commands: Commands,
    mut q_dialogue: Query<&mut Visibility, With<Dialogue>>,
    mut q_dialogue_runners: Query<(Entity, &mut DialogueRunner)>,
    mut ev_player_stopped_chat: EventReader<PlayerStoppedChat>,
) {
    let mut visibility = match q_dialogue.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if ev_player_stopped_chat.is_empty() {
        return;
    }
    ev_player_stopped_chat.clear();

    *visibility = Visibility::Hidden;
    for (entity, mut runner) in &mut q_dialogue_runners {
        runner.stop();
        commands.entity(entity).despawn_recursive();
    }
}

pub struct DialogueRunnerPlugin;

impl Plugin for DialogueRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_dialogue_runner, despawn_dialogue_runner)
                .run_if(in_state(GameState::Gaming).and_then(resource_exists::<YarnProject>())),
        );
    }
}
