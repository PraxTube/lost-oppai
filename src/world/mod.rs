pub mod camera;
pub mod camera_shake;
pub mod ending;
pub mod map;

pub use camera::MainCamera;
// pub use camera_shake::CameraShake;

use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::dynamics::IntegrationParameters};

use crate::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            camera_shake::CameraShakePlugin,
            map::MapPlugin,
            ending::EndingPlugin,
        ))
        .add_systems(OnExit(GameState::AssetLoading), configure_physics);
    }
}

fn configure_physics(
    mut rapier_config: ResMut<RapierConfiguration>,
    mut rapier_context: ResMut<RapierContext>,
) {
    rapier_config.gravity = Vec2::ZERO;
    rapier_context.integration_parameters = IntegrationParameters {
        normalized_max_corrective_velocity: f32::MAX,
        contact_damping_ratio: 1.0,
        ..default()
    };
}
