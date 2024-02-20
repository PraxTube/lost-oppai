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
