use bevy::prelude::*;
use bevy_tweening::{lens::*, *};

use crate::GameState;

const FADE_OUT_DURATION: f32 = 3.0;
const START_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 1.0);
const END_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.0);

fn fade_out_intro_screen(mut commands: Commands) {
    let tween = Tween::new(
        EaseFunction::CubicIn,
        std::time::Duration::from_secs_f32(FADE_OUT_DURATION),
        UiBackgroundColorLens {
            start: START_COLOR,
            end: END_COLOR,
        },
    );

    commands.spawn((
        Animator::new(tween),
        ImageBundle {
            style: Style {
                width: Val::Vw(110.0),
                height: Val::Vh(110.0),
                ..default()
            },
            ..default()
        },
    ));
}

pub struct ScreenFadeUiPlugin;

impl Plugin for ScreenFadeUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), (fade_out_intro_screen,));
    }
}
