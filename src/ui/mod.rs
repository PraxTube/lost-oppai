pub mod dialogue;

mod keyboard_hint;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((dialogue::DialoguePlugin, keyboard_hint::KeyboardUiPlugin));
    }
}
