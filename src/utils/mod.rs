mod debug;

pub use debug::DebugActive;

use bevy::prelude::*;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(debug::DebugPlugin);
    }
}
