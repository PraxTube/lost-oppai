pub mod dialogue;
pub mod keyboard_hint;

mod audio_bar;
mod ending_text;
mod main_menu;
mod screen_fade;
mod splash_screen;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            dialogue::DialoguePlugin,
            keyboard_hint::KeyboardUiPlugin,
            screen_fade::ScreenFadeUiPlugin,
            audio_bar::AudioBarPlugin,
            ending_text::EndingTextPlugin,
            splash_screen::SplashScreenPlugin,
            main_menu::MainMenuPlugin,
        ));
    }
}
