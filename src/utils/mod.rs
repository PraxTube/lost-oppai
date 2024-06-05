use std::iter::Sum;
use std::ops::Add;

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

pub struct FixedQueue<T, const N: usize> {
    buffer: [Option<T>; N],
    head: usize,
}

impl<T, const N: usize> FixedQueue<T, N>
where
    T: Add<Output = T> + Copy + Default + Sum,
{
    pub fn new() -> Self {
        Self {
            buffer: [(); N].map(|_| None),
            head: 0,
        }
    }

    pub fn add(&mut self, value: T) {
        self.buffer[self.head] = Some(value);
        self.head = (self.head + 1) % N;
    }

    pub fn compute_average(&self) -> Option<T> {
        let mut result: Option<T> = None;
        for value in self.buffer {
            if result.is_none() {
                result = value;
                continue;
            }

            if let (Some(r), Some(v)) = (result, value) {
                result = Some(r + v);
            }
        }
        result
    }
}
