use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;

use super::dialogue::Typewriter;
use crate::{GameAssets, GameState};

const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

const DIALOGUE_LINE: &str = "Example dialogue. Select speed at which to display dialogue in game. You can also disable blip sounds.";

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
enum ButtonAction {
    Normal,
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
        font_size: 23.0,
        color: Color::WHITE,
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
        font_size: 23.0,
        color: Color::WHITE,
        font: assets.pixel_font.clone(),
    };

    let text = commands
        .spawn(
            TextBundle::from_section("Enable Blips", button_text_style).with_style(Style {
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
            parent.spawn(ImageBundle {
                image: UiImage {
                    texture: assets.ui_tick.clone(),
                    ..default()
                },
                ..default()
            });
        })
        .id();

    commands
        .spawn((NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },))
        .push_children(&[text, button])
        .id()
}

fn spawn_main_menu(mut commands: Commands, assets: Res<GameAssets>) {
    let normal_dialogue_button =
        spawn_button(&mut commands, &assets, ButtonAction::Normal, "Normal");
    let fast_dialogue_button = spawn_button(&mut commands, &assets, ButtonAction::Fast, "Fast");
    let instant_dialogue_button =
        spawn_button(&mut commands, &assets, ButtonAction::Instant, "Instant");
    let box_button = spawn_box_button(&mut commands, &assets);
    let play_button = spawn_button(&mut commands, &assets, ButtonAction::Play, "Play");

    let speed_buttons = commands
        .spawn(NodeBundle {
            style: Style {
                column_gap: Val::Px(100.0),
                ..default()
            },
            ..default()
        })
        .push_children(&[
            normal_dialogue_button,
            fast_dialogue_button,
            instant_dialogue_button,
        ])
        .id();

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                top: Val::Px(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .push_children(&[speed_buttons, box_button, play_button]);
}

fn highlight_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut UiImage, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut image, selected) in &mut interaction_query {
        image.color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON,
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON,
            (Interaction::Hovered, None) => HOVERED_BUTTON,
            (Interaction::None, None) => Color::WHITE,
        }
    }
}

fn insert_typewriter_line(mut typewriter: ResMut<Typewriter>) {
    typewriter.reset();
    typewriter.set_line(&LocalizedLine {
        id: LineId(String::new()),
        text: DIALOGUE_LINE.to_string(),
        attributes: Vec::new(),
        metadata: Vec::new(),
        assets: LineAssets::new(),
    });
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), (spawn_main_menu,))
            .add_systems(OnEnter(GameState::MainMenu), insert_typewriter_line)
            .add_systems(
                Update,
                (highlight_buttons).run_if(in_state(GameState::MainMenu)),
            );
    }
}
