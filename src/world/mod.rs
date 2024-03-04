pub mod camera;
pub mod camera_shake;
pub mod map;

pub use camera::MainCamera;
// pub use camera_shake::CameraShake;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            camera_shake::CameraShakePlugin,
            map::MapPlugin,
        ))
        .add_systems(OnExit(GameState::AssetLoading), configure_physics);
    }
}

fn configure_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
