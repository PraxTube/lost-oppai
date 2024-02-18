use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_screen_diagnostics::{
    ScreenDiagnostics, ScreenDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin,
    ScreenFrameDiagnosticsPlugin,
};

use crate::{player::input::PlayerInput, GameState};

#[derive(Resource, Default, Deref, DerefMut)]
pub struct DebugActive(pub bool);

#[derive(Resource, Default)]
struct Diagnostics {
    active: bool,
}

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

fn toggle_diags(screen_diags: &mut ResMut<ScreenDiagnostics>) {
    screen_diags.modify("fps").toggle();
    screen_diags.modify("ms/frame").toggle();
    screen_diags.modify("entities").toggle();
}

fn toggle_diagnostics_off(mut screen_diags: ResMut<ScreenDiagnostics>) {
    if false {
        toggle_diags(&mut screen_diags);
    }
}

fn toggle_diagnostics(
    debug_active: Res<DebugActive>,
    mut diags: ResMut<Diagnostics>,
    mut screen_diags: ResMut<ScreenDiagnostics>,
) {
    if **debug_active != diags.active {
        diags.active = **debug_active;
        toggle_diags(&mut screen_diags);
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ScreenDiagnosticsPlugin {
                timestep: 1.0,
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            ScreenFrameDiagnosticsPlugin,
            ScreenEntityDiagnosticsPlugin,
        ))
        .init_resource::<DebugActive>()
        .init_resource::<Diagnostics>()
        .add_systems(Update, (toggle_debug_mod, toggle_rapier_debug))
        .add_systems(OnExit(GameState::AssetLoading), toggle_diagnostics_off)
        .add_systems(
            Update,
            toggle_diagnostics.run_if(resource_changed::<DebugActive>()),
        );
    }
}
