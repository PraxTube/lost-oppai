use bevy::prelude::*;

use super::typewriter::Typewriter;

pub fn set_type_speed(In(speed): In<f32>, mut typewriter: ResMut<Typewriter>) {
    typewriter.set_type_speed(speed);
}
