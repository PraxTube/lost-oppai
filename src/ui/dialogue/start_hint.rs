use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    npc::Npc,
    player::{chat::PlayerStartedChat, Player, PlayerState, NPC_PROXIMITY_DISTANCE},
    world::camera::YSort,
    GameAssets, GameState,
};

#[derive(Component)]
struct StartHint;

fn spawn_hint(
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
            let mut animator = AnimationPlayer2D::default();
            animator
                .play(assets.dialogue_start_hint_animations[0].clone())
                .repeat();

            let sprite = commands
                .spawn((
                    animator,
                    SpriteSheetBundle {
                        texture_atlas: assets.dialogue_start_hint.clone(),
                        transform: Transform::from_translation(Vec3::new(0.0, 40.0, 0.0))
                            .with_scale(Vec3::splat(0.65)),
                        ..default()
                    },
                ))
                .id();

            commands
                .spawn((
                    StartHint,
                    YSort(100.0),
                    SpatialBundle {
                        transform: Transform::from_translation(npc_transform.translation),
                        ..default()
                    },
                ))
                .push_children(&[sprite]);
        }
        break;
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
            (spawn_hint, despawn_hint).run_if(in_state(GameState::Gaming)),
        );
    }
}
