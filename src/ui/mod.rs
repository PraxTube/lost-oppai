mod dialogue;

use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;

use crate::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((dialogue::DialoguePlugin,))
            .add_systems(OnExit(GameState::AssetLoading), (spawn_dialogue_runner,));
    }
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.create_dialogue_runner();
    dialogue_runner.start_node("HelloWorld");
    commands.spawn(dialogue_runner);
}
