use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::once_after_delay};
use bevy_tweening::{lens::TransformScaleLens, Animator, EaseFunction, Tween};
use bevy_yarnspinner::prelude::*;

use super::dialogue::Typewriter;
use crate::{audio::PlaySound, GameAssets, GameState};

const SCALE_TWEEN_TIME: f32 = 0.4;
const NORMAL_SCALE: f32 = 1.0;
const HOVERED_SCALE: f32 = 0.9;
const PRESSED_SCALE: f32 = 0.8;
const SPAWN_DELAY: f32 = 0.5;
const DIALOGUE_LINE: &str = "Example dialogue. Select speed at which to display dialogue in game. You can also click the button in the bottom right corner to join my discord server ^-^";
const DISCORD_LINK: &str = "https://discord.gg/2h7dncQNTr";

#[derive(Event)]
pub struct MainMenuButtonPressed(pub ButtonAction);

#[derive(Component)]
struct MainMenuUiRoot;
#[derive(Component)]
struct SelectedOption;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum ButtonAction {
    Normal,
    Quick,
    Fast,
    Instant,
    Play,
    Discord,
}

#[derive(Component)]
struct DiscordButton {
    timer: Timer,
    delay: Timer,
}

fn spawn_button(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    action: ButtonAction,
    title: &str,
) -> Entity {
    let button_style = Style {
        width: Val::Px(192.0),
        height: Val::Px(96.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        color: Color::WHITE,
        font_size: 23.0,
        font: assets.pixel_font.clone(),
    };

    let text = commands
        .spawn(TextBundle::from_section(title, button_text_style))
        .id();

    commands
        .spawn((
            ButtonBundle {
                style: button_style,
                image: UiImage {
                    texture: assets.button.clone(),
                    ..default()
                },
                ..default()
            },
            action,
        ))
        .add_child(text)
        .id()
}

fn spawn_discord_button(commands: &mut Commands, assets: &Res<GameAssets>) {
    let style = Style {
        width: Val::Px(100.0),
        height: Val::Px(100.0),
        bottom: Val::Px(20.0),
        right: Val::Px(20.0),
        position_type: PositionType::Absolute,
        ..default()
    };

    commands.spawn((
        ButtonAction::Discord,
        DiscordButton {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
            delay: Timer::from_seconds(1.5, TimerMode::Once),
        },
        ButtonBundle {
            style,
            image: UiImage {
                color: Color::WHITE.with_alpha(0.0),
                texture: assets.discord_button.clone(),
                ..default()
            },
            ..default()
        },
    ));
}

fn spawn_main_menu(mut commands: Commands, assets: Res<GameAssets>) {
    let entity = spawn_button(&mut commands, &assets, ButtonAction::Normal, "Normal");
    let tween = Tween::new(
        EaseFunction::ExponentialOut,
        Duration::from_secs_f32(SCALE_TWEEN_TIME),
        TransformScaleLens {
            start: Vec3::splat(HOVERED_SCALE),
            end: Vec3::splat(PRESSED_SCALE),
        },
    );
    let normal_dialogue_button = commands
        .entity(entity)
        .insert((Animator::new(tween), SelectedOption))
        .id();
    let quick_dialogue_button = spawn_button(&mut commands, &assets, ButtonAction::Quick, "Quick");
    let fast_dialogue_button = spawn_button(&mut commands, &assets, ButtonAction::Fast, "Fast");
    let instant_dialogue_button =
        spawn_button(&mut commands, &assets, ButtonAction::Instant, "Instant");
    let play_button = spawn_button(&mut commands, &assets, ButtonAction::Play, "Play");
    spawn_discord_button(&mut commands, &assets);

    let speed_buttons = commands
        .spawn(NodeBundle {
            style: Style {
                column_gap: Val::Px(50.0),
                ..default()
            },
            ..default()
        })
        .push_children(&[
            normal_dialogue_button,
            quick_dialogue_button,
            fast_dialogue_button,
            instant_dialogue_button,
        ])
        .id();

    let tween = Tween::new(
        EaseFunction::ExponentialIn,
        Duration::from_secs_f32(SPAWN_DELAY),
        TransformScaleLens {
            start: Vec3::ZERO,
            end: Vec3::ONE,
        },
    );

    commands
        .spawn((
            MainMenuUiRoot,
            Animator::new(tween),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    top: Val::Px(150.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
        ))
        .push_children(&[speed_buttons, play_button]);
}

fn highlight_buttons(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut interaction_query: Query<
        (Entity, &Interaction, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    for (entity, interaction, selected) in &mut interaction_query {
        let tween = match (*interaction, selected) {
            (_, Some(_)) => continue,
            (Interaction::Pressed, None) => {
                ev_play_sound.send(PlaySound {
                    clip: assets.ui_button_press_sound.clone(),
                    volume: 0.5,
                    ..default()
                });
                Tween::new(
                    EaseFunction::ExponentialOut,
                    Duration::from_secs_f32(SCALE_TWEEN_TIME),
                    TransformScaleLens {
                        start: Vec3::splat(HOVERED_SCALE),
                        end: Vec3::splat(PRESSED_SCALE),
                    },
                )
            }
            (Interaction::Hovered, None) => {
                ev_play_sound.send(PlaySound {
                    clip: assets.ui_button_hover_sound.clone(),
                    volume: 0.5,
                    ..default()
                });
                Tween::new(
                    EaseFunction::ExponentialOut,
                    Duration::from_secs_f32(SCALE_TWEEN_TIME),
                    TransformScaleLens {
                        start: Vec3::splat(NORMAL_SCALE),
                        end: Vec3::splat(HOVERED_SCALE),
                    },
                )
            }
            (Interaction::None, None) => Tween::new(
                EaseFunction::ExponentialOut,
                Duration::from_secs_f32(SCALE_TWEEN_TIME),
                TransformScaleLens {
                    start: Vec3::splat(HOVERED_SCALE),
                    end: Vec3::splat(NORMAL_SCALE),
                },
            ),
        };
        commands.entity(entity).insert(Animator::new(tween));
    }
}

fn dehighlight_buttons(
    mut commands: Commands,
    q_buttons: Query<(Entity, &ButtonAction, Option<&SelectedOption>), With<Button>>,
    mut ev_main_menu_button_pressed: EventReader<MainMenuButtonPressed>,
) {
    for ev in ev_main_menu_button_pressed.read() {
        match ev.0 {
            ButtonAction::Normal
            | ButtonAction::Quick
            | ButtonAction::Fast
            | ButtonAction::Instant => {}
            _ => continue,
        }

        for (entity, action, selected) in &q_buttons {
            if selected.is_none() {
                continue;
            }

            if *action != ev.0 {
                let tween = Tween::new(
                    EaseFunction::ExponentialOut,
                    Duration::from_secs_f32(SCALE_TWEEN_TIME),
                    TransformScaleLens {
                        start: Vec3::splat(PRESSED_SCALE),
                        end: Vec3::splat(NORMAL_SCALE),
                    },
                );
                commands
                    .entity(entity)
                    .remove::<SelectedOption>()
                    .insert(Animator::new(tween));
            }
        }
    }
}

fn trigger_button_actions(
    mut commands: Commands,
    interaction_query: Query<
        (Entity, &Interaction, &ButtonAction, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
    mut ev_main_menu_button_pressed: EventWriter<MainMenuButtonPressed>,
) {
    for (entity, interaction, action, selected) in &interaction_query {
        if selected.is_some() {
            continue;
        }
        if *interaction == Interaction::Pressed {
            match action {
                ButtonAction::Normal
                | ButtonAction::Quick
                | ButtonAction::Fast
                | ButtonAction::Instant => {
                    commands.entity(entity).insert(SelectedOption);
                }
                _ => {}
            }
            ev_main_menu_button_pressed.send(MainMenuButtonPressed(*action));
        }
    }
}

fn set_typewriter(typewriter: &mut ResMut<Typewriter>) {
    typewriter.reset();
    typewriter.set_line(&LocalizedLine {
        id: LineId(String::new()),
        text: DIALOGUE_LINE.to_string(),
        attributes: Vec::new(),
        metadata: Vec::new(),
        assets: LineAssets::new(),
    });
}

fn insert_typewriter_line(mut typewriter: ResMut<Typewriter>) {
    set_typewriter(&mut typewriter);
}

fn change_to_playing_game_state(
    mut next_state: ResMut<NextState<GameState>>,
    mut ev_main_menu_button_pressed: EventReader<MainMenuButtonPressed>,
) {
    for ev in ev_main_menu_button_pressed.read() {
        if ev.0 == ButtonAction::Play {
            next_state.set(GameState::Gaming);
        }
    }
}

fn reset_typewriter_line(
    mut typewriter: ResMut<Typewriter>,
    mut ev_main_menu_button_pressed: EventReader<MainMenuButtonPressed>,
) {
    for ev in ev_main_menu_button_pressed.read() {
        match ev.0 {
            ButtonAction::Normal
            | ButtonAction::Quick
            | ButtonAction::Fast
            | ButtonAction::Instant => set_typewriter(&mut typewriter),
            _ => {}
        }
    }
}

fn reset_typewriter(mut typewriter: ResMut<Typewriter>) {
    typewriter.reset();
}

fn despawn_main_menu(
    mut commands: Commands,
    q_main_menu: Query<Entity, Or<(With<MainMenuUiRoot>, With<DiscordButton>)>>,
) {
    for entity in &q_main_menu {
        commands.entity(entity).despawn_recursive();
    }
}

fn animate_discord_button(
    time: Res<Time>,
    mut q_discord_button: Query<(&mut UiImage, &mut DiscordButton)>,
) {
    let Ok((mut image, mut discord_button)) = q_discord_button.get_single_mut() else {
        return;
    };

    if !discord_button.delay.finished() {
        discord_button.delay.tick(time.delta());
        return;
    }

    discord_button.timer.tick(time.delta());

    if discord_button.timer.just_finished() {
        image.color.set_alpha(1.0);
        return;
    }

    if discord_button.timer.finished() {
        return;
    }

    let alpha = discord_button.timer.elapsed_secs() / discord_button.timer.duration().as_secs_f32();
    image.color.set_alpha(alpha);
}

fn open_discord_link(mut ev_button_pressed: EventReader<MainMenuButtonPressed>) {
    if !ev_button_pressed
        .read()
        .any(|ev| ev.0 == ButtonAction::Discord)
    {
        return;
    }

    let err_str = "failed to open discord link in default browser";

    #[cfg(not(target_arch = "wasm32"))]
    if open::that(DISCORD_LINK).is_err() {
        error!(err_str);
    }

    #[cfg(target_arch = "wasm32")]
    if let Some(win) = web_sys::window() {
        if win
            .open_with_url_and_target(DISCORD_LINK, "_blank")
            .is_err()
        {
            error!(err_str);
        }
    }
}

fn set_button_action_normal_on_delay(
    mut ev_main_menu_button_pressed: EventWriter<MainMenuButtonPressed>,
) {
    ev_main_menu_button_pressed.send(MainMenuButtonPressed(ButtonAction::Normal));
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MainMenuButtonPressed>()
            .add_systems(OnEnter(GameState::MainMenu), (spawn_main_menu,))
            .add_systems(
                Update,
                (
                    highlight_buttons,
                    dehighlight_buttons,
                    trigger_button_actions,
                    reset_typewriter_line,
                    change_to_playing_game_state,
                    insert_typewriter_line
                        .run_if(once_after_delay(Duration::from_secs_f32(SPAWN_DELAY))),
                    animate_discord_button,
                    open_discord_link,
                    set_button_action_normal_on_delay
                        .run_if(once_after_delay(Duration::from_secs_f32(SPAWN_DELAY + 0.1))),
                )
                    .run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(
                OnExit(GameState::MainMenu),
                (despawn_main_menu, reset_typewriter),
            );
    }
}
