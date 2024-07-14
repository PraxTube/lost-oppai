use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    world::{
        camera::{YSort, YSortChild},
        map::generation::BitMap,
    },
    GameAssets, GameState,
};

use super::{Npc, NpcDialogue};

fn spawn_eleonore(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec2) {
    let transform = Transform::from_translation(pos.extend(0.0));
    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.eleonore_animations[0].clone())
        .repeat();

    let mut shadow_animator = AnimationPlayer2D::default();
    shadow_animator
        .play(assets.eleonore_animations[0].clone())
        .repeat();

    let shadow = commands
        .spawn((
            YSortChild(-26.0),
            shadow_animator,
            SpriteSheetBundle {
                transform: Transform::from_translation(Vec3::new(0.0, -25.0, 0.0)),
                texture_atlas: assets.eleonore_shadow.clone(),
                ..default()
            },
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::ball(16.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::default(),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -20.0, 0.0,
            ))),
        ))
        .id();

    commands
        .spawn((
            Npc::new(NpcDialogue::Eleonore),
            YSort(16.0),
            animator,
            SpriteSheetBundle {
                transform,
                texture_atlas: assets.eleonore.clone(),
                ..default()
            },
        ))
        .push_children(&[collider, shadow]);
}

fn spawn_joanna(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec2) {
    let transform = Transform::from_translation(pos.extend(0.0));
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.joanna_animatins[0].clone()).repeat();

    let shadow = commands
        .spawn((
            YSortChild(-22.0),
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(0.0, -21.0, 0.0)),
                texture: assets.joanna_shadow.clone(),
                ..default()
            },
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::ball(16.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::default(),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -16.0, 0.0,
            ))),
        ))
        .id();

    commands
        .spawn((
            Npc::new(NpcDialogue::Joanna),
            YSort(0.0),
            animator,
            SpriteSheetBundle {
                transform,
                texture_atlas: assets.joanna.clone(),
                ..default()
            },
        ))
        .push_children(&[collider, shadow]);
}

fn spawn_jotem(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec2) {
    let transform = Transform::from_translation(pos.extend(0.0));
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.jotem_animations[0].clone()).repeat();

    let shadow = commands
        .spawn((
            YSortChild(-19.0),
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(0.0, -18.0, 0.0)),
                texture: assets.jotem_shadow.clone(),
                ..default()
            },
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::ball(16.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::default(),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -16.0, 0.0,
            ))),
        ))
        .id();

    commands
        .spawn((
            Npc::new(NpcDialogue::Jotem),
            YSort(0.0),
            animator,
            SpriteSheetBundle {
                transform,
                texture_atlas: assets.jotem.clone(),
                ..default()
            },
        ))
        .push_children(&[collider, shadow]);
}

fn spawn_isabelle(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec2) {
    let transform = Transform::from_translation(pos.extend(0.0));
    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.isabelle_animations[0].clone())
        .repeat();

    let collider = commands
        .spawn((
            Collider::ball(16.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::default(),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -16.0, 0.0,
            ))),
        ))
        .id();

    commands
        .spawn((
            Npc::new(NpcDialogue::Isabelle),
            YSort(0.0),
            animator,
            SpriteSheetBundle {
                transform,
                texture_atlas: assets.isabelle.clone(),
                ..default()
            },
        ))
        .push_children(&[collider]);
}

fn spawn_antonius(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec2) -> Entity {
    let transform = Transform::from_translation(pos.extend(0.0));
    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.antonius_animations[0].clone())
        .repeat();

    let shadow = commands
        .spawn((
            YSortChild(-1.0),
            SpriteBundle {
                texture: assets.antonius_shadow.clone(),
                ..default()
            },
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::ball(8.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::default(),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -16.0, 0.0,
            ))),
        ))
        .id();

    commands
        .spawn((
            animator,
            SpriteSheetBundle {
                transform,
                texture_atlas: assets.antonius.clone(),
                ..default()
            },
        ))
        .push_children(&[collider, shadow])
        .id()
}

fn spawn_ionas(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec2) -> Entity {
    let transform = Transform::from_translation(pos.extend(0.0));
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.ionas_animations[0].clone()).repeat();

    let shadow = commands
        .spawn((
            YSortChild(-1.0),
            SpriteBundle {
                texture: assets.antonius_shadow.clone(),
                ..default()
            },
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::ball(8.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::default(),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -16.0, 0.0,
            ))),
        ))
        .id();

    commands
        .spawn((
            animator,
            SpriteSheetBundle {
                transform,
                texture_atlas: assets.ionas.clone(),
                sprite: TextureAtlasSprite {
                    flip_x: true,
                    ..default()
                },
                ..default()
            },
        ))
        .push_children(&[collider, shadow])
        .id()
}

fn spawn_antonius_and_ionas(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec2) {
    let offset = Vec2::new(-20.0, 0.0);
    let antonius = spawn_antonius(commands, assets, offset);
    let ionas = spawn_ionas(commands, assets, -offset);

    commands
        .spawn((
            YSort(0.0),
            Npc::new(NpcDialogue::IonasAndAntonius),
            SpatialBundle {
                transform: Transform::from_translation(pos.extend(0.0)),
                ..default()
            },
        ))
        .push_children(&[antonius, ionas]);
}

fn spawn_npcs(mut commands: Commands, bitmap: Res<BitMap>, assets: Res<GameAssets>) {
    spawn_eleonore(&mut commands, &assets, Vec2::new(-200.0, -200.0));
    spawn_joanna(&mut commands, &assets, bitmap.get_hotspot(1));
    spawn_jotem(&mut commands, &assets, Vec2::ZERO);
    spawn_isabelle(&mut commands, &assets, bitmap.get_hotspot(2));
    spawn_antonius_and_ionas(&mut commands, &assets, Vec2::new(200.0, 200.0));
}

pub struct NpcSpawnPlugin;

impl Plugin for NpcSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), (spawn_npcs,));
    }
}
