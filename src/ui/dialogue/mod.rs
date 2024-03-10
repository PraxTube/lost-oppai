mod option_selection;
mod setup;
mod typewriter;
mod updating;

use bevy::prelude::*;

/// The plugin registering all systems of the dialogue view.
#[derive(Debug, Default)]
// #[non_exhaustive]
pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            setup::DialogueSetupPlugin,
            updating::DialogueUpdatingPlugin,
            option_selection::DialogueSelectionPlugin,
            typewriter::DialogueTypewriterPlugin,
        ));
    }
}

#[derive(Debug, Default, Clone, Copy, SystemSet, Eq, PartialEq, Hash)]
pub struct DialogueViewSystemSet;
