use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;

use crate::{GameAssets, GameState};

use super::option_selection::OptionSelection;

#[derive(Default, Component)]
pub struct DialogueRoot;
#[derive(Default, Component)]
pub struct DialogueContent;
#[derive(Default, Component)]
pub struct DialogueNameNode;
#[derive(Default, Component)]
pub struct DialogueContinueNode;

#[derive(Component)]
pub struct OptionsNode;
#[derive(Component)]
pub struct OptionsBackground;
#[derive(Component)]
pub struct OptionButton(pub OptionId);

pub const INITIAL_DIALOGUE_CONTINUE_BOTTOM: f32 = -5.0;
const DIALOG_WIDTH: f32 = 800.0 * 0.8;
const OPTIONS_WIDTH: f32 = 420.0;
const TEXT_BORDER: f32 = 120.0;
const OPTIONS_TEXT_BORDER: f32 = 10.0;

fn style_standard(_assets: &Res<GameAssets>) -> Style {
    Style {
        max_width: Val::Px(DIALOG_WIDTH - 2.0 * TEXT_BORDER),
        ..default()
    }
}

fn text_style_standard(_assets: &Res<GameAssets>) -> TextStyle {
    TextStyle {
        font_size: 24.0,
        color: Color::WHITE,
        ..default()
    }
}

fn text_style_name(_assets: &Res<GameAssets>) -> TextStyle {
    TextStyle {
        font_size: 22.0,
        color: Color::WHITE,
        ..default()
    }
}

fn text_style_option_text(_assets: &Res<GameAssets>) -> TextStyle {
    TextStyle {
        font_size: 24.0,
        color: Color::WHITE,
        ..default()
    }
}

fn spawn_dialogue_top(commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
    let edge = commands
        .spawn((ImageBundle {
            image: UiImage {
                texture: assets.dialogue_edge.clone(),
                ..default()
            },
            style: Style {
                width: Val::Px(DIALOG_WIDTH),
                ..default()
            },
            ..default()
        },))
        .id();

    let name_node = commands
        .spawn((
            TextBundle {
                text: Text::from_section(String::new(), text_style_name(assets)),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(TEXT_BORDER / 2.0),
                    top: Val::Px(-8.0),
                    ..default()
                },
                z_index: ZIndex::Local(1),
                ..default()
            },
            DialogueNameNode,
            Label,
        ))
        .id();

    commands
        .spawn((NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },))
        .push_children(&[edge, name_node])
        .id()
}

fn spawn_dialogue_content(commands: &mut Commands, _assets: &Res<GameAssets>) -> Entity {
    let text = commands
        .spawn((
            TextBundle::from_section(String::new(), text_style_standard(_assets))
                .with_style(style_standard(_assets)),
            DialogueContent,
            Label,
        ))
        .id();

    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Px(DIALOG_WIDTH),
                min_height: Val::Px(50.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::FlexStart,
                padding: UiRect::horizontal(Val::Px(TEXT_BORDER)),
                ..default()
            },
            background_color: Color::BLACK.with_a(0.8).into(),
            ..default()
        },))
        .push_children(&[text])
        .id()
}

fn spawn_dialogue_bottom(commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
    let edge = commands
        .spawn((ImageBundle {
            image: UiImage {
                texture: assets.dialogue_edge.clone(),
                flip_y: true,
                ..default()
            },
            style: Style {
                width: Val::Px(DIALOG_WIDTH),
                ..default()
            },
            ..default()
        },))
        .id();

    let continue_node = commands
        .spawn((
            ImageBundle {
                image: UiImage {
                    texture: assets.dialogue_continue.clone(),
                    ..default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(INITIAL_DIALOGUE_CONTINUE_BOTTOM),
                    ..default()
                },
                z_index: ZIndex::Local(1),
                visibility: Visibility::Hidden,
                ..default()
            },
            DialogueContinueNode,
        ))
        .id();

    commands
        .spawn((NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },))
        .push_children(&[edge, continue_node])
        .id()
}

fn spawn_dialogue_options(commands: &mut Commands, _assets: &Res<GameAssets>) {
    let options = commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::None,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                ..default()
            },
            OptionsNode,
        ))
        .id();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(OPTIONS_WIDTH),
                    min_height: Val::Px(100.0),
                    max_height: Val::Px(720.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::FlexStart,
                    padding: UiRect::all(Val::Px(20.0)),
                    top: Val::Percent(0.0),
                    right: Val::Percent(0.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::BLACK.with_a(0.8).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            OptionsBackground,
        ))
        .push_children(&[options]);
}

fn spawn_dialogue(mut commands: Commands, assets: Res<GameAssets>) {
    spawn_dialogue_options(&mut commands, &assets);

    let dialogue_top = spawn_dialogue_top(&mut commands, &assets);
    let dialogue_content = spawn_dialogue_content(&mut commands, &assets);
    let dialogue_bottom = spawn_dialogue_bottom(&mut commands, &assets);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::Center,
                    padding: UiRect::bottom(Val::Px(30.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            DialogueRoot,
        ))
        .push_children(&[dialogue_top, dialogue_content, dialogue_bottom]);
}

pub fn create_dialogue_text(
    text: impl Into<String>,
    invisible: impl Into<String>,
    assets: &Res<GameAssets>,
) -> Text {
    Text::from_sections([
        TextSection {
            value: text.into(),
            style: text_style_standard(assets),
        },
        TextSection {
            value: invisible.into(),
            style: TextStyle {
                color: Color::NONE,
                ..text_style_standard(assets)
            },
        },
    ])
}

pub fn spawn_options(
    commands: &mut Commands,
    options: &Res<OptionSelection>,
    assets: &Res<GameAssets>,
    entity: Entity,
) {
    for option in options.get_options() {
        let sections = [TextSection {
            value: format!("- {}", option.line.text.clone()),
            style: text_style_option_text(assets),
        }];
        let text = commands
            .spawn((
                TextBundle::from_sections(sections).with_style(Style {
                    width: Val::Px(OPTIONS_WIDTH - 2.0 * OPTIONS_TEXT_BORDER),
                    ..default()
                }),
                Label,
            ))
            .id();

        let button = commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        justify_content: JustifyContent::FlexStart,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                },
                OptionButton(option.id),
            ))
            .push_children(&[text])
            .id();

        commands.entity(entity).push_children(&[button]);
    }
}

pub struct DialogueSpawnPlugin;

impl Plugin for DialogueSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), spawn_dialogue);
    }
}
