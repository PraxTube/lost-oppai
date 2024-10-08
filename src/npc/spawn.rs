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
            SpriteBundle {
                texture: assets.eleonore_shadow_texture.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, -25.0, 0.0)),
                ..default()
            },
            TextureAtlas {
                layout: assets.eleonore_shadow_layout.clone(),
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
            SpriteBundle {
                texture: assets.eleonore_texture.clone(),
                transform,
                ..default()
            },
            TextureAtlas {
                layout: assets.eleonore_layout.clone(),
                ..default()
            },
        ))
        .push_children(&[collider, shadow]);
}

fn spawn_jotem(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec2) {
    let transform = Transform::from_translation(pos.extend(0.0));
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.jotem_animations[0].clone()).repeat();

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
            SpriteBundle {
                texture: assets.jotem_texture.clone(),
                transform,
                ..default()
            },
            TextureAtlas {
                layout: assets.jotem_layout.clone(),
                ..default()
            },
        ))
        .push_children(&[collider]);
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
            SpriteBundle {
                texture: assets.isabelle_texture.clone(),
                transform,
                ..default()
            },
            TextureAtlas {
                layout: assets.isabelle_layout.clone(),
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
            SpriteBundle {
                texture: assets.antonius_texture.clone(),
                transform,
                ..default()
            },
            TextureAtlas {
                layout: assets.antonius_layout.clone(),
                ..default()
            },
        ))
        .push_children(&[collider])
        .id()
}

fn spawn_ionas(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec2) -> Entity {
    let transform = Transform::from_translation(pos.extend(0.0));
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.ionas_animations[0].clone()).repeat();

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
            SpriteBundle {
                texture: assets.ionas_texture.clone(),
                transform,
                sprite: Sprite {
                    flip_x: true,
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: assets.ionas_layout.clone(),
                ..default()
            },
        ))
        .push_children(&[collider])
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
    let hotspots = bitmap.get_furthest_hotspots(4);
    let npcs = [
        spawn_eleonore,
        spawn_jotem,
        spawn_isabelle,
        spawn_antonius_and_ionas,
    ];

    for i in 0..hotspots.len() {
        npcs[i](&mut commands, &assets, hotspots[i]);
    }
}

pub struct NpcSpawnPlugin;

impl Plugin for NpcSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), (spawn_npcs,));
    }
}
