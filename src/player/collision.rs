use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::utils::{DebugActive, COLLISION_GROUPS_NONE};

use super::{Player, PLAYER_COLLISION_GROUPS};

fn toggle_player_collision(
    debug_active: Res<DebugActive>,
    mut q_player: Query<&Children, With<Player>>,
    mut q_colliders: Query<&mut CollisionGroups>,
) {
    let children = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    for child in children {
        if let Ok(mut c) = q_colliders.get_mut(*child) {
            *c = if **debug_active {
                COLLISION_GROUPS_NONE
            } else {
                PLAYER_COLLISION_GROUPS
            };
            break;
        };
    }
}

pub struct PlayerCollisionPlugin;

impl Plugin for PlayerCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (toggle_player_collision).run_if(resource_changed::<DebugActive>()),
        );
    }
}
