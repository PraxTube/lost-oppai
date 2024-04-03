use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    world::{camera::YSort, map::generation::BitMap},
    GameAssets, GameState,
};

const ANCHOR_DIS: f32 = 100.0;
const BUTTON_DIS: f32 = 40.0;
const ARROW_DIS: f32 = 80.0;
const SHIFT_DIS: f32 = 60.0;
const ICON_SIZE: f32 = 0.5;

#[derive(Component)]
struct KeyboardIcon;

enum Icon {
    DownKey,
    UpKey,
    LeftKey,
    RightKey,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    ShiftKey,
}

fn icon_to_texture(assets: &Res<GameAssets>, icon: &Icon) -> Handle<TextureAtlas> {
    match icon {
        Icon::DownKey => assets.ui_down_key.clone(),
        Icon::DownArrow => assets.ui_down_key.clone(),
        Icon::UpKey => assets.ui_up_key.clone(),
        Icon::UpArrow => assets.ui_up_key.clone(),
        Icon::LeftKey => assets.ui_left_key.clone(),
        Icon::LeftArrow => assets.ui_left_key.clone(),
        Icon::RightKey => assets.ui_right_key.clone(),
        Icon::RightArrow => assets.ui_right_key.clone(),
        Icon::ShiftKey => assets.ui_shift_key.clone(),
    }
}

fn spawn_icon(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    root: Entity,
    icon: Icon,
    offset: Vec2,
) -> Entity {
    let texture_atlas = icon_to_texture(assets, &icon);
    let transform = Transform::from_translation(offset.extend(0.0));

    let icon = commands
        .spawn((
            KeyboardIcon,
            SpriteSheetBundle {
                transform,
                texture_atlas,
                ..default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[icon]);
    icon
}

fn spawn_animated_icon(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    root: Entity,
    icon: Icon,
    offset: Vec2,
) {
    let entity = spawn_icon(commands, assets, root, icon, offset);

    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.ui_keys_animations[0].clone()).repeat();

    commands
        .entity(entity)
        .insert((Collider::cuboid(16.0, 16.0), animator));
}

fn spawn_unanimated_icon(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    root: Entity,
    icon: Icon,
    offset: Vec2,
) {
    let entity = spawn_icon(commands, assets, root, icon, offset);

    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.ui_keys_animations[1].clone()).repeat();

    commands.entity(entity).insert(animator);
}

fn spawn_shift_icon(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    root: Entity,
    icon: Icon,
    offset: Vec2,
) {
    let entity = spawn_icon(commands, assets, root, icon, offset);

    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.ui_keys_animations[2].clone()).repeat();

    commands
        .entity(entity)
        .insert((Collider::cuboid(32.0, 16.0), animator));
}

fn spawn_keyboard_ui(mut commands: Commands, assets: Res<GameAssets>, bitmap: Res<BitMap>) {
    let transform = Transform::from_translation(
        -bitmap.center_point().normalize_or_zero().extend(0.0) * ANCHOR_DIS,
    )
    .with_scale(Vec3::splat(ICON_SIZE));
    let root = commands
        .spawn((YSort(-200.0), SpatialBundle::from_transform(transform)))
        .id();

    spawn_animated_icon(
        &mut commands,
        &assets,
        root,
        Icon::DownKey,
        Vec2::new(0.0, -BUTTON_DIS),
    );
    spawn_unanimated_icon(
        &mut commands,
        &assets,
        root,
        Icon::DownArrow,
        Vec2::new(0.0, -ARROW_DIS),
    );
    spawn_animated_icon(
        &mut commands,
        &assets,
        root,
        Icon::UpKey,
        Vec2::new(0.0, BUTTON_DIS),
    );
    spawn_unanimated_icon(
        &mut commands,
        &assets,
        root,
        Icon::UpArrow,
        Vec2::new(0.0, ARROW_DIS),
    );
    spawn_animated_icon(
        &mut commands,
        &assets,
        root,
        Icon::LeftKey,
        Vec2::new(-BUTTON_DIS, 0.0),
    );
    spawn_unanimated_icon(
        &mut commands,
        &assets,
        root,
        Icon::LeftArrow,
        Vec2::new(-ARROW_DIS, 0.0),
    );
    spawn_animated_icon(
        &mut commands,
        &assets,
        root,
        Icon::RightKey,
        Vec2::new(BUTTON_DIS, 0.0),
    );
    spawn_unanimated_icon(
        &mut commands,
        &assets,
        root,
        Icon::RightArrow,
        Vec2::new(ARROW_DIS, 0.0),
    );
    spawn_shift_icon(
        &mut commands,
        &assets,
        root,
        Icon::ShiftKey,
        Vec2::new(SHIFT_DIS, SHIFT_DIS),
    );
}

pub struct KeyboardUiPlugin;

impl Plugin for KeyboardUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), spawn_keyboard_ui);
    }
}
