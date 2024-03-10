use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;

use crate::{GameAssets, GameState};

#[derive(Debug, Default, Component)]
pub struct UiRootNode;

#[derive(Debug, Default, Component)]
pub struct DialogueNode;

#[derive(Debug, Default, Component)]
pub struct DialogueNameNode;

#[derive(Debug, Default, Component)]
pub struct DialogueContinueNode;

#[derive(Debug, Default, Component)]
pub struct OptionsNode;

#[derive(Debug, Component)]
pub struct OptionButton(pub OptionId);

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
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
            UiRootNode,
        ))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn((ImageBundle {
                        image: UiImage {
                            texture: assets.dialogue_edge.clone(),
                            ..default()
                        },
                        style: Style {
                            width: Val::Px(DIALOG_WIDTH),
                            ..default()
                        },
                        ..default()
                    },));
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(String::new(), text_style::name()),
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
                    ));
                });

            parent
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
                .with_children(|parent| {
                    // Dialog itself
                    parent.spawn((
                        TextBundle::from_section(String::new(), text_style::standard())
                            .with_style(style::standard()),
                        DialogueNode,
                        Label,
                    ));
                })
                .with_children(|parent| {
                    // Options
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                display: Display::None,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::FlexEnd,
                                align_items: AlignItems::FlexStart,
                                margin: UiRect::top(Val::Px(20.0)),
                                ..default()
                            },
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        OptionsNode,
                    ));
                });

            parent
                .spawn((NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn((ImageBundle {
                        image: UiImage {
                            // 29 pixels high
                            texture: assets.dialogue_edge.clone(),
                            flip_y: true,
                            ..default()
                        },
                        style: Style {
                            width: Val::Px(DIALOG_WIDTH),
                            ..default()
                        },
                        ..default()
                    },));
                })
                .with_children(|parent| {
                    parent.spawn((
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
                    ));
                });
        });
}

pub const INITIAL_DIALOGUE_CONTINUE_BOTTOM: f32 = -5.0;

pub fn create_dialog_text(text: impl Into<String>, invisible: impl Into<String>) -> Text {
    Text::from_sections([
        TextSection {
            value: text.into(),
            style: text_style::standard(),
        },
        TextSection {
            value: invisible.into(),
            style: TextStyle {
                color: Color::NONE,
                ..text_style::standard()
            },
        },
    ])
}

pub fn spawn_options<'a, T>(entity_commands: &mut EntityCommands, options: T)
where
    T: IntoIterator<Item = &'a DialogueOption>,
    <T as IntoIterator>::IntoIter: 'a,
{
    entity_commands.with_children(|parent| {
        for (i, option) in options.into_iter().enumerate() {
            parent
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
                .with_children(|parent| {
                    let sections = [
                        TextSection {
                            value: format!("{}: ", i + 1),
                            style: text_style::option_id(),
                        },
                        TextSection {
                            value: option.line.text.clone(),
                            style: text_style::option_text(),
                        },
                    ];

                    parent.spawn((
                        TextBundle::from_sections(sections).with_style(style::options()),
                        Label,
                    ));
                });
        }
    });
}

const DIALOG_WIDTH: f32 = 800.0 * 0.8;
const TEXT_BORDER: f32 = 120.0;

mod style {
    use super::*;
    pub fn standard() -> Style {
        Style {
            max_width: Val::Px(DIALOG_WIDTH - 2.0 * TEXT_BORDER),
            ..default()
        }
    }
    pub fn options() -> Style {
        const INDENT_MODIFIER: f32 = 1.0;
        Style {
            margin: UiRect::horizontal(Val::Px((INDENT_MODIFIER - 1.0) * TEXT_BORDER)),
            max_width: Val::Px(DIALOG_WIDTH - 2.0 * INDENT_MODIFIER * TEXT_BORDER),
            ..default()
        }
    }
}

mod text_style {
    use super::*;
    pub fn standard() -> TextStyle {
        TextStyle {
            font_size: 20.0,
            color: Color::WHITE,
            ..default()
        }
    }
    pub fn name() -> TextStyle {
        TextStyle {
            font_size: 18.0,
            ..standard()
        }
    }

    pub fn option_id() -> TextStyle {
        TextStyle {
            color: Color::ALICE_BLUE,
            ..option_text()
        }
    }

    pub fn option_text() -> TextStyle {
        TextStyle {
            font_size: 18.0,
            color: Color::TOMATO,
            ..standard()
        }
    }
}

pub struct DialogueSetupPlugin;

impl Plugin for DialogueSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), setup);
    }
}
