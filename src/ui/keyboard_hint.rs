use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    world::{camera::YSort, map::generation::BitMap},
    GameAssets, GameState,
};

const ANCHOR_DIS: f32 = 90.0;
const BUTTON_DIS: f32 = 35.0;
const SHIFT_DIS: f32 = 35.0;
const ICON_SIZE: f32 = 0.5;

pub const KEYBOARD_ICON_RADIUS: f32 = 100.0;

#[derive(Component)]
struct KeyboardIcon;
#[derive(Component)]
pub struct KeyboardHint;

enum Icon {
    Down,
    Up,
    Left,
    Right,
    Shift,
}

fn icon_to_texture(
    assets: &Res<GameAssets>,
    icon: &Icon,
) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
    match icon {
        Icon::Down => (
            assets.ui_down_key_texture.clone(),
            assets.ui_down_key_layout.clone(),
        ),
        Icon::Up => (
            assets.ui_up_key_texture.clone(),
            assets.ui_up_key_layout.clone(),
        ),
        Icon::Left => (
            assets.ui_left_key_texture.clone(),
            assets.ui_left_key_layout.clone(),
        ),
        Icon::Right => (
            assets.ui_right_key_texture.clone(),
            assets.ui_right_key_layout.clone(),
        ),
        Icon::Shift => (
            assets.ui_shift_key_texture.clone(),
            assets.ui_shift_key_layout.clone(),
        ),
    }
}

fn spawn_icon(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    root: Entity,
    icon: Icon,
    offset: Vec2,
) -> Entity {
    let (texture, layout) = icon_to_texture(assets, &icon);
    let transform = Transform::from_translation(offset.extend(0.0));

    let icon = commands
        .spawn((
            KeyboardIcon,
            SpriteBundle {
                texture,
                transform,
                ..default()
            },
            TextureAtlas {
                layout,
                ..default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[icon]);
    icon
}

fn spawn_key_buttons(
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

fn spawn_arrows_icon(commands: &mut Commands, assets: &Res<GameAssets>, root: Entity) {
    let icon = commands
        .spawn((
            KeyboardIcon,
            SpriteBundle {
                texture: assets.ui_arrows_key.clone(),
                transform: Transform::from_scale(Vec3::splat(1.0)),
                ..default()
            },
        ))
        .id();

    commands.entity(root).add_child(icon);
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
    animator.play(assets.ui_keys_animations[1].clone()).repeat();

    commands
        .entity(entity)
        .insert((Collider::cuboid(16.0, 16.0), animator));
}

fn calculate_dir(vecs: &Vec<Vec2>) -> Vec2 {
    if vecs.is_empty() {
        warn!("There is no connection between the origin and any other vertex. \
            Should never happen, something must be wrong with the edge algorithm (kruskals algorithm)");
        return Vec2::ZERO;
    }
    if vecs.len() == 1 {
        return -vecs[0];
    }

    // Convert the vectors to polar coordinates with only the angle
    let mut angles = Vec::new();
    for v in vecs {
        let angle = v.y.atan2(v.x);
        let angle = if angle < 0.0 { TAU + angle } else { angle };
        angles.push(angle);
    }
    // Sort the angles so that we get the vectors in a (a)cylcic order
    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Initialize the max_angle and current_angle to the biggest - small angle
    // This is useful because we would need to handle this in a special way anyways
    let mut max_angle = angles[0] + TAU - angles[angles.len() - 1];
    let mut current_angle = angles[angles.len() - 1];
    for i in 0..angles.len() - 1 {
        let angle_between = angles[i + 1] - angles[i];
        if angle_between > max_angle {
            max_angle = angle_between;
            current_angle = angles[i];
        }
    }

    // Finally compute the direction angle by taking
    // the vector (here as an angle) with the corresponding biggest between angle
    let final_angle = current_angle + max_angle / 2.0;
    Vec2::from_angle(final_angle)
}

fn spawn_keyboard_ui(mut commands: Commands, assets: Res<GameAssets>, bitmap: Res<BitMap>) {
    let transform = Transform::from_translation(
        calculate_dir(&bitmap.get_origin_edges())
            .normalize_or_zero()
            .extend(0.0)
            * ANCHOR_DIS,
    )
    .with_scale(Vec3::splat(ICON_SIZE));
    let root = commands
        .spawn((
            KeyboardHint,
            YSort(-200.0),
            SpatialBundle::from_transform(transform),
        ))
        .id();

    spawn_key_buttons(
        &mut commands,
        &assets,
        root,
        Icon::Down,
        Vec2::new(0.0, -BUTTON_DIS),
    );
    spawn_key_buttons(
        &mut commands,
        &assets,
        root,
        Icon::Up,
        Vec2::new(0.0, BUTTON_DIS),
    );
    spawn_key_buttons(
        &mut commands,
        &assets,
        root,
        Icon::Left,
        Vec2::new(-BUTTON_DIS, 0.0),
    );
    spawn_key_buttons(
        &mut commands,
        &assets,
        root,
        Icon::Right,
        Vec2::new(BUTTON_DIS, 0.0),
    );
    spawn_arrows_icon(&mut commands, &assets, root);
    spawn_shift_icon(
        &mut commands,
        &assets,
        root,
        Icon::Shift,
        Vec2::new(SHIFT_DIS, SHIFT_DIS),
    );
}

pub struct KeyboardUiPlugin;

impl Plugin for KeyboardUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_keyboard_ui);
    }
}
