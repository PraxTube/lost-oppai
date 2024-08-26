use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::once_after_delay};
use bevy_tweening::{lens::TransformScaleLens, Animator, EaseFunction, Tween};
use bevy_yarnspinner::prelude::*;

use super::dialogue::Typewriter;
use crate::{audio::PlaySound, player::input::PlayerInput, GameAssets, GameState};

const SCALE_TWEEN_TIME: f32 = 0.4;
const NORMAL_SCALE: f32 = 1.0;
const HOVERED_SCALE: f32 = 0.9;
const PRESSED_SCALE: f32 = 0.8;
const SPAWN_DELAY: f32 = 0.5;
const DIALOGUE_LINE: &str = "Example dialogue. Select speed at which to display dialogue in game. You can also disable dialogue blip sounds.";

#[derive(Event)]
pub struct MainMenuButtonPressed(pub ButtonAction);

#[derive(Component)]
struct MainMenuUiRoot;
#[derive(Component)]
struct SelectedOption;
#[derive(Component)]
struct BoxMarker(bool);

#[derive(Component, Clone, Copy, PartialEq)]
pub enum ButtonAction {
    Normal,
    Quick,
    Fast,
    Instant,
    Play,
    ToggleBlips,
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

fn spawn_box_button(commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
    let button_text_style = TextStyle {
        color: Color::WHITE,
        font_size: 23.0,
        font: assets.pixel_font.clone(),
    };

    let text = commands
        .spawn(
            TextBundle::from_section("Dialogue Blip Sounds", button_text_style).with_style(Style {
                margin: UiRect::right(Val::Px(30.0)),
                ..default()
            }),
        )
        .id();
    let button = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(64.0),
                    height: Val::Px(64.0),
                    ..default()
                },
                image: UiImage {
                    texture: assets.box_button.clone(),
                    ..default()
                },
                ..default()
            },
            ButtonAction::ToggleBlips,
        ))
        .with_children(|parent| {
            parent.spawn((
                BoxMarker(true),
                ImageBundle {
                    image: UiImage {
                        texture: assets.ui_tick.clone(),
                        ..default()
                    },
                    ..default()
                },
            ));
        })
        .id();

    commands
        .spawn((NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            },
            ..default()
        },))
        .push_children(&[text, button])
        .id()
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
    let box_button = spawn_box_button(&mut commands, &assets);
    let play_button = spawn_button(&mut commands, &assets, ButtonAction::Play, "Play");

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
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    top: Val::Px(130.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
        ))
        .push_children(&[speed_buttons, box_button, play_button]);
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

fn update_box_marker(
    assets: Res<GameAssets>,
    mut q_box_marker: Query<(&mut UiImage, &mut BoxMarker)>,
    mut ev_main_menu_button_pressed: EventReader<MainMenuButtonPressed>,
) {
    let Ok((mut image, mut box_marker)) = q_box_marker.get_single_mut() else {
        return;
    };

    for ev in ev_main_menu_button_pressed.read() {
        if ev.0 != ButtonAction::ToggleBlips {
            continue;
        }

        if box_marker.0 {
            image.texture = assets.ui_cross.clone();
        } else {
            image.texture = assets.ui_tick.clone();
        }
        box_marker.0 = !box_marker.0;
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

fn confirm_main_menu(
    player_input: Res<PlayerInput>,
    mut ev_main_menu_button_pressed: EventWriter<MainMenuButtonPressed>,
) {
    if player_input.dialogue_confirm {
        ev_main_menu_button_pressed.send(MainMenuButtonPressed(ButtonAction::Play));
    }
}

fn reset_typewriter(mut typewriter: ResMut<Typewriter>) {
    typewriter.reset();
}

fn despawn_main_menu(mut commands: Commands, q_main_menu: Query<Entity, With<MainMenuUiRoot>>) {
    for entity in &q_main_menu {
        commands.entity(entity).despawn_recursive();
    }
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
                    update_box_marker,
                    insert_typewriter_line
                        .run_if(once_after_delay(Duration::from_secs_f32(SPAWN_DELAY))),
                    confirm_main_menu,
                )
                    .run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(
                OnExit(GameState::MainMenu),
                (despawn_main_menu, reset_typewriter),
            );
    }
}
