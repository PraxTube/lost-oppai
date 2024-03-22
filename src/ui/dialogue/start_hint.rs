use bevy::prelude::*;
use bevy_trickfilm::prelude::*;
use bevy_tweening::{lens::*, *};

use crate::{
    npc::Npc,
    player::{chat::PlayerStartedChat, Player, PlayerState, NPC_PROXIMITY_DISTANCE},
    world::camera::YSort,
    GameAssets, GameState,
};

const SIZE: f32 = 0.65;
const SCALE_DURATION: f32 = 0.5;

#[derive(Component)]
struct StartHint;

fn spawn_hint(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3) {
    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.dialogue_start_hint_animations[0].clone())
        .repeat();

    let tween = Tween::new(
        EaseFunction::QuarticInOut,
        std::time::Duration::from_secs_f32(SCALE_DURATION),
        TransformScaleLens {
            start: Vec3::ZERO,
            end: Vec3::ONE * SIZE,
        },
    );

    let sprite = commands
        .spawn((
            animator,
            Animator::new(tween),
            SpriteSheetBundle {
                texture_atlas: assets.dialogue_start_hint.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, 40.0, 0.0))
                    .with_scale(Vec3::ZERO),
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            StartHint,
            YSort(100.0),
            SpatialBundle {
                transform: Transform::from_translation(pos),
                ..default()
            },
        ))
        .push_children(&[sprite]);
}

fn spawn_hints(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<(&Transform, &Player)>,
    q_npcs: Query<&Transform, (With<Npc>, Without<Player>)>,
    q_start_hints: Query<With<StartHint>>,
) {
    if !q_start_hints.is_empty() {
        return;
    }

    let (player_transform, player) = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if player.state == PlayerState::Talking {
        return;
    }

    for npc_transform in &q_npcs {
        if player_transform
            .translation
            .xy()
            .distance_squared(npc_transform.translation.xy())
            <= NPC_PROXIMITY_DISTANCE.powi(2)
        {
            spawn_hint(&mut commands, &assets, npc_transform.translation);
        }
    }
}

fn despawn_hint(
    mut commands: Commands,
    q_player: Query<(&Transform, &Player)>,
    q_start_hints: Query<(Entity, &Transform), (With<StartHint>, Without<Player>)>,
    mut ev_player_started_chat: EventReader<PlayerStartedChat>,
) {
    if !ev_player_started_chat.is_empty() {
        ev_player_started_chat.clear();
        for (entity, _) in &q_start_hints {
            commands.entity(entity).despawn_recursive();
        }
        return;
    }

    if q_start_hints.is_empty() {
        return;
    }

    let (player_transform, player) = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if player.state == PlayerState::Talking {
        for (entity, _) in &q_start_hints {
            commands.entity(entity).despawn_recursive();
        }
        return;
    }

    for (entity, transform) in &q_start_hints {
        if player_transform
            .translation
            .xy()
            .distance_squared(transform.translation.xy())
            > NPC_PROXIMITY_DISTANCE.powi(2)
        {
            commands.entity(entity).despawn_recursive();
            break;
        }
    }
}

pub struct DialogueStartHintPlugin;

impl Plugin for DialogueStartHintPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_hints, despawn_hint).run_if(in_state(GameState::Gaming)),
        );
    }
}
