pub mod dialogue;
pub mod keyboard_hint;

mod audio_bar;
mod screen_fade;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            dialogue::DialoguePlugin,
            keyboard_hint::KeyboardUiPlugin,
            screen_fade::ScreenFadeUiPlugin,
            audio_bar::AudioBarPlugin,
        ));
    }
}
