pub mod runner;

mod audio;
mod command;
mod option_selection;
mod spawn;
mod start_hint;
mod typewriter;
mod updating;

use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn::DialogueSpawnPlugin,
            updating::DialogueUpdatingPlugin,
            option_selection::DialogueSelectionPlugin,
            typewriter::DialogueTypewriterPlugin,
            runner::DialogueRunnerPlugin,
            start_hint::DialogueStartHintPlugin,
            audio::DialogueAudioPlugin,
            command::DialogueCommandPlugin,
        ));
    }
}

#[derive(Debug, Default, Clone, Copy, SystemSet, Eq, PartialEq, Hash)]
pub struct DialogueViewSystemSet;
