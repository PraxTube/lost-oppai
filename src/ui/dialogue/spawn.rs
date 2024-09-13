use bevy::prelude::*;
use bevy_tweening::{lens::*, *};
use bevy_yarnspinner::prelude::*;

use crate::{GameAssets, GameState};

use super::option_selection::OptionSelection;

// The master root of the dialogue
#[derive(Component)]
pub struct DialogueRoot;
#[derive(Component)]
pub struct DialogueContent;
#[derive(Component)]
pub struct DialogueNameNode;
#[derive(Component)]
pub struct DialogueCharacterIcon;
#[derive(Component)]
pub struct DialogueContinueNode;

#[derive(Component)]
pub struct OptionsNode;
#[derive(Component)]
pub struct OptionsBackground;
#[derive(Component)]
pub struct OptionsText;
#[derive(Component)]
pub struct OptionButton(pub OptionId);

const BACKGROUND_ALPHA: f32 = 0.9;
const DIALOG_WIDTH: f32 = 800.0;
const OPTIONS_WIDTH: f32 = 500.0;
const TEXT_BORDER_HORIZONTAL: f32 = 40.0;
const TEXT_BORDER_VERTICAL: f32 = 20.0;
const OPTIONS_TEXT_BORDER: f32 = 10.0;

const CONTINUE_BOTTOM: f32 = -5.0;
const CONTINUE_BOB_DURATION: f32 = 1.0;
const CONTINUE_BOB_OFFSET: f32 = 5.0;

fn style_standard(_assets: &Res<GameAssets>) -> Style {
    Style {
        max_width: Val::Px(DIALOG_WIDTH - 2.0 * TEXT_BORDER_HORIZONTAL),
        ..default()
    }
}

fn text_style_standard(assets: &Res<GameAssets>) -> TextStyle {
    TextStyle {
        font: assets.silver_font.clone(),
        font_size: 50.0,
        color: Color::WHITE,
    }
}

fn text_style_name(assets: &Res<GameAssets>) -> TextStyle {
    TextStyle {
        font: assets.silver_font.clone(),
        font_size: 46.0,
        color: Color::WHITE,
    }
}

fn text_style_option_text(assets: &Res<GameAssets>) -> TextStyle {
    TextStyle {
        font: assets.silver_font.clone(),
        font_size: 50.0,
        color: Color::WHITE,
    }
}

fn spawn_dialogue_content(commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
    let text = commands
        .spawn((
            DialogueContent,
            Label,
            TextBundle::from_section(String::new(), text_style_standard(assets))
                .with_style(style_standard(assets)),
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
                padding: UiRect {
                    left: Val::Px(TEXT_BORDER_HORIZONTAL),
                    right: Val::Px(TEXT_BORDER_HORIZONTAL),
                    top: Val::Px(TEXT_BORDER_VERTICAL),
                    bottom: Val::Px(TEXT_BORDER_VERTICAL),
                },
                ..default()
            },
            background_color: Color::BLACK.with_alpha(BACKGROUND_ALPHA).into(),
            ..default()
        },))
        .push_children(&[text])
        .id()
}

fn spawn_dialogue_bottom(commands: &mut Commands, assets: &Res<GameAssets>) -> Entity {
    let tween = Tween::new(
        EaseFunction::SineInOut,
        std::time::Duration::from_secs_f32(CONTINUE_BOB_DURATION),
        UiPositionLens {
            start: UiRect::new(
                Val::Auto,
                Val::Auto,
                Val::Auto,
                Val::Px(CONTINUE_BOTTOM + CONTINUE_BOB_OFFSET),
            ),
            end: UiRect::new(
                Val::Auto,
                Val::Auto,
                Val::Auto,
                Val::Px(CONTINUE_BOTTOM - CONTINUE_BOB_OFFSET),
            ),
        },
    )
    .with_repeat_count(RepeatCount::Infinite)
    .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

    let continue_node = commands
        .spawn((
            DialogueContinueNode,
            Animator::new(tween),
            ImageBundle {
                image: UiImage {
                    texture: assets.dialogue_continue.clone(),
                    ..default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                z_index: ZIndex::Local(1),
                visibility: Visibility::Hidden,
                ..default()
            },
        ))
        .id();

    let icon_node = commands
        .spawn((
            ImageBundle {
                image: UiImage::new(assets.eleonore_icon.clone()),
                style: Style {
                    width: Val::Px(128.0),
                    height: Val::Px(128.0),
                    ..default()
                },
                z_index: ZIndex::Local(1),
                ..default()
            },
            DialogueCharacterIcon,
        ))
        .id();

    let name_node = commands
        .spawn((
            TextBundle {
                text: Text::from_section(String::new(), text_style_name(assets)),
                style: Style {
                    // bottom: Val::Px(-25.0),
                    ..default()
                },
                z_index: ZIndex::Local(1),
                ..default()
            },
            DialogueNameNode,
            Label,
        ))
        .id();

    let character_icon_node = commands
        .spawn((NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(-550.0),
                bottom: Val::Px(0.0),
                width: Val::Px(140.0),
                height: Val::Px(180.0),
                // justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            z_index: ZIndex::Local(1),
            background_color: Color::BLACK.with_alpha(BACKGROUND_ALPHA).into(),
            ..default()
        },))
        .push_children(&[icon_node, name_node])
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
        .push_children(&[character_icon_node, continue_node])
        .id()
}

fn spawn_dialogue_options(commands: &mut Commands, _assets: &Res<GameAssets>) -> Entity {
    let options = commands
        .spawn((
            OptionsNode,
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            OptionsBackground,
            NodeBundle {
                style: Style {
                    width: Val::Px(OPTIONS_WIDTH),
                    min_height: Val::Px(100.0),
                    max_height: Val::Px(720.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::FlexStart,
                    padding: UiRect {
                        top: Val::Px(20.0),
                        bottom: Val::Px(20.0),
                        left: Val::Px(20.0),
                        right: Val::Px(OPTIONS_WIDTH + 10.0),
                    },
                    top: Val::Percent(0.0),
                    right: Val::Percent(0.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::BLACK.with_alpha(BACKGROUND_ALPHA).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
        ))
        .push_children(&[options])
        .id()
}

fn spawn_dialogue(mut commands: Commands, assets: Res<GameAssets>) {
    let dialogue_options = spawn_dialogue_options(&mut commands, &assets);

    let dialogue_content = spawn_dialogue_content(&mut commands, &assets);
    let dialogue_bottom = spawn_dialogue_bottom(&mut commands, &assets);

    let dialogue_root = commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                padding: UiRect::bottom(Val::Px(30.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },))
        .push_children(&[dialogue_content, dialogue_bottom])
        .id();

    commands
        .spawn((
            DialogueRoot,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
        ))
        .push_children(&[dialogue_options, dialogue_root]);
}

fn spawn_dialogue_main_menu(mut commands: Commands, assets: Res<GameAssets>) {
    let dialogue_content = spawn_dialogue_content(&mut commands, &assets);

    let dialogue_root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::top(Val::Px(30.0)),
                flex_direction: FlexDirection::Column,
                top: Val::Px(-200.0),
                ..default()
            },
            ..default()
        })
        .add_child(dialogue_content)
        .id();

    commands
        .spawn((
            DialogueRoot,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
        ))
        .add_child(dialogue_root);
}

fn despawn_dialogue_main_menu(
    mut commands: Commands,
    q_dialogue_root: Query<Entity, With<DialogueRoot>>,
) {
    for entity in &q_dialogue_root {
        commands.entity(entity).despawn_recursive();
    }
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

fn spawn_option(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    option: DialogueOption,
    entity: Entity,
) {
    let sections = [TextSection {
        value: format!("- {}", option.line.text.clone()),
        style: text_style_option_text(assets),
    }];
    let text = commands
        .spawn((
            OptionsText,
            Label,
            TextBundle::from_sections(sections).with_style(Style {
                width: Val::Px(OPTIONS_WIDTH - 2.0 * OPTIONS_TEXT_BORDER),
                ..default()
            }),
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

pub fn spawn_options(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    options: &OptionSelection,
    entity: Entity,
) {
    for option in options.get_options() {
        spawn_option(commands, assets, option, entity);
    }
}

pub struct DialogueSpawnPlugin;

impl Plugin for DialogueSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_dialogue_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_dialogue_main_menu)
            .add_systems(OnEnter(GameState::Gaming), spawn_dialogue);
    }
}
