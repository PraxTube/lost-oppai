mod debug;

pub use debug::DebugActive;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const COLLISION_GROUPS_NONE: CollisionGroups = CollisionGroups::new(Group::NONE, Group::NONE);

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(debug::DebugPlugin);
    }
}

#[allow(dead_code)]
pub fn quat_from_vec2(direction: Vec2) -> Quat {
    if direction == Vec2::ZERO {
        return Quat::IDENTITY;
    }
    Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, Vec2::X.angle_between(direction))
}

#[allow(dead_code)]
pub fn quat_from_vec3(direction: Vec3) -> Quat {
    quat_from_vec2(direction.truncate())
}
