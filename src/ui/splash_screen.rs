use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::TransformScaleLens, Animator, EaseFunction, Tween, TweenCompleted};

use crate::GameState;

const SPASH_SCREEN_TWEEN_ID: u64 = 100;

#[derive(Component)]
struct SplashScreen;

fn spawn_splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("ui/bevy_icon.png");
    let image = commands
        .spawn(ImageBundle {
            image: UiImage::new(icon),
            style: Style {
                width: Val::Px(200.0),
                ..default()
            },
            z_index: ZIndex::Global(1000),
            ..default()
        })
        .id();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            SplashScreen,
        ))
        .add_child(image);
}

fn fade_out_splash_screen(
    mut commands: Commands,
    q_splash_screen: Query<Entity, With<SplashScreen>>,
) {
    for entity in &q_splash_screen {
        let tween = Tween::new(
            EaseFunction::ExponentialIn,
            Duration::from_secs_f32(0.5),
            TransformScaleLens {
                start: Vec3::ONE,
                end: Vec3::ZERO,
            },
        )
        .with_completed_event(SPASH_SCREEN_TWEEN_ID);
        commands.entity(entity).insert(Animator::new(tween));
    }
}

fn despawn_splash_screen(
    mut commands: Commands,
    mut ev_tween_completed: EventReader<TweenCompleted>,
) {
    for ev in ev_tween_completed.read() {
        if ev.user_data == SPASH_SCREEN_TWEEN_ID {
            commands.entity(ev.entity).despawn_recursive();
        }
    }
}

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::AssetLoading), spawn_splash_screen)
            .add_systems(OnExit(GameState::AssetLoading), fade_out_splash_screen)
            .add_systems(Update, (despawn_splash_screen,));
    }
}
