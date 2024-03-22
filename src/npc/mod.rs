use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    player::Player,
    world::{camera::YSort, map::generation::BitMap},
    GameAssets, GameState,
};

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), (spawn_npcs,))
            .add_systems(Update, (face_player,));
    }
}

#[derive(Component)]
pub struct Npc {
    pub dialogue: String,
}

impl Npc {
    fn new(dialogue: &str) -> Self {
        Self {
            dialogue: dialogue.to_string(),
        }
    }
}

fn spawn_npc(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3, npc: &str) {
    let transform = Transform::from_translation(pos);
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.npc_animations[0].clone()).repeat();

    let mut shadow_animator = AnimationPlayer2D::default();
    shadow_animator
        .play(assets.npc_animations[0].clone())
        .repeat();

    let shadow = commands
        .spawn((
            YSort(-256.0),
            shadow_animator,
            SpriteSheetBundle {
                transform: Transform::from_translation(Vec3::new(0.0, -25.0, 0.0)),
                texture_atlas: assets.npc_shadow.clone(),
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
            Npc::new(npc),
            YSort(16.0),
            animator,
            SpriteSheetBundle {
                transform,
                texture_atlas: assets.npc.clone(),
                ..default()
            },
        ))
        .push_children(&[collider, shadow]);
}

fn spawn_npcs(mut commands: Commands, bitmap: Res<BitMap>, assets: Res<GameAssets>) {
    let pos = [bitmap.center_point().extend(0.0), Vec3::new(-5.0, 3.0, 0.0)];
    let npcs = ["Eleonore", "NOTEleonore"];
    for i in 0..pos.len() {
        spawn_npc(&mut commands, &assets, pos[i], npcs[i]);
    }
}

fn face_player(
    q_player: Query<&Transform, With<Player>>,
    mut q_npcs: Query<(&Transform, &mut TextureAtlasSprite), (With<Npc>, Without<Player>)>,
) {
    let player = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    for (transform, mut sprite) in &mut q_npcs {
        let flip = player.translation.x < transform.translation.x;
        sprite.flip_x = flip;
    }
}
