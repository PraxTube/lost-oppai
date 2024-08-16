pub mod dialogue;
pub mod keyboard_hint;

mod audio_bar;
mod ending_text;
mod main_menu;
mod screen_fade;
mod splash_screen;

use bevy::{prelude::*, window::WindowResized};

use crate::DEFAULT_WINDOW_WIDTH;

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
        ))
        .add_systems(Update, scale_ui);
    }
}

fn scale_ui(mut ui_scale: ResMut<UiScale>, mut ev_window_resized: EventReader<WindowResized>) {
    for ev in ev_window_resized.read() {
        ui_scale.0 = ev.width / DEFAULT_WINDOW_WIDTH;
    }
}
