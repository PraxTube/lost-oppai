use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::input::PlayerInput;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct DebugActive(pub bool);

fn toggle_debug_mod(player_input: Res<PlayerInput>, mut debug_active: ResMut<DebugActive>) {
    if player_input.toggle_debug {
        **debug_active = !**debug_active;
    }
}

fn toggle_rapier_debug(
    mut debug_context: ResMut<DebugRenderContext>,
    debug_active: Res<DebugActive>,
) {
    if debug_context.enabled != **debug_active {
        debug_context.enabled = **debug_active;
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugActive>()
            .add_systems(Update, (toggle_debug_mod, toggle_rapier_debug));
    }
}
