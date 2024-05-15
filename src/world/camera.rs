use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
#[cfg(not(target_arch = "wasm32"))]
use bevy::render::view::screenshot::ScreenshotManager;
use bevy::window::{PrimaryWindow, WindowMode};
// use bevy_kira_audio::prelude::AudioReceiver;
use bevy_rapier2d::dynamics::Velocity;

use super::camera_shake::{update_camera, CameraShake};
use crate::player::input::PlayerInput;
use crate::player::Player;
use crate::utils::DebugActive;
use crate::GameState;

// Only relevant for the backend.
// We have to multiply each z coordinate with this value
// because camera rendering only works for entities that are
// at most 1000 z coordinates away.
// Too small values may lead to float inpercision errors,
// too large values will lead to overflow of the 1000 range
// (in which case they won't get rendered on the camera anymore).
const YSORT_SCALE: f32 = 0.0001;
// This is not changing the actual timestep,
// it's just a way to reduce magic numbers in code.
const RAPIER_TIMESTEP: f32 = 60.0;
const PROJECTION_SCALE: f32 = 200.0;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct YSort(pub f32);

#[derive(Component)]
pub struct YSortStatic(pub f32);
#[derive(Component)]
pub struct YSortStaticChild(pub f32);

pub fn apply_y_sort(mut q_transforms: Query<(&mut Transform, &GlobalTransform, &YSort)>) {
    for (mut transform, global_transform, ysort) in &mut q_transforms {
        transform.translation.z = (ysort.0 - global_transform.translation().y) * YSORT_SCALE;
    }
}

pub fn apply_y_sort_static(
    mut q_transforms: Query<(&mut Transform, &GlobalTransform, &YSortStatic), Added<YSortStatic>>,
) {
    for (mut transform, global_transform, ysort) in &mut q_transforms {
        transform.translation.z = (ysort.0 - global_transform.translation().y) * YSORT_SCALE;
    }
}

pub fn apply_y_sort_static_child(
    q_parents: Query<&Transform, (With<YSortStatic>, Without<YSortStaticChild>)>,
    mut q_transforms: Query<
        (&Parent, &mut Transform, &GlobalTransform, &YSortStaticChild),
        (Added<YSortStaticChild>, Without<YSortStatic>),
    >,
) {
    for (parent, mut transform, global_transform, ysort) in &mut q_transforms {
        let parent_transform = match q_parents.get(parent.get()) {
            Ok(r) => r,
            Err(_) => continue,
        };
        transform.translation.z = (ysort.0 - global_transform.translation().y) * YSORT_SCALE
            - parent_transform.translation.z;
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(PROJECTION_SCALE);
    // commands.spawn((MainCamera, AudioReceiver, camera));
    commands.spawn((MainCamera, camera));
}

fn update_camera_target(
    mut shake: ResMut<CameraShake>,
    q_player: Query<(&Transform, &Velocity), With<Player>>,
) {
    let (player_pos, player_vel) = match q_player.get_single() {
        Ok(p) => (p.0.translation, p.1),
        Err(_) => return,
    };

    shake.update_target(Vec2::new(
        player_pos.x + player_vel.linvel.x / RAPIER_TIMESTEP,
        player_pos.y + player_vel.linvel.y / RAPIER_TIMESTEP,
    ));
}

fn zoom_camera(
    player_input: Res<PlayerInput>,
    debug_active: Res<DebugActive>,
    mut q_projection: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    if !**debug_active {
        return;
    }

    let mut projection = match q_projection.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    projection.scale = (projection.scale + player_input.zoom).clamp(1.0, 10.0);
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_full_screen(
    mut main_window: Query<&mut Window, With<PrimaryWindow>>,
    player_input: Res<PlayerInput>,
) {
    if !player_input.toggle_fullscreen {
        return;
    }

    let mut window = match main_window.get_single_mut() {
        Ok(w) => w,
        Err(err) => {
            error!("there is not exactly one window, {}", err);
            return;
        }
    };

    window.mode = if window.mode == WindowMode::Windowed {
        WindowMode::Fullscreen
    } else {
        WindowMode::Windowed
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn take_screenshot(
    keys: Res<Input<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut counter: Local<u32>,
) {
    if !keys.just_pressed(KeyCode::F12) {
        return;
    }

    let path = format!("./screenshot-{}.png", *counter);
    *counter += 1;
    match screenshot_manager.save_screenshot_to_disk(main_window.single(), path) {
        Ok(()) => {}
        Err(err) => error!("failed to take screenshot, {}", err),
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                #[cfg(not(target_arch = "wasm32"))]
                toggle_full_screen,
                #[cfg(not(target_arch = "wasm32"))]
                take_screenshot,
                apply_y_sort,
                apply_y_sort_static.after(apply_y_sort),
                apply_y_sort_static_child.after(apply_y_sort_static),
                zoom_camera,
            ),
        )
        .add_systems(OnEnter(GameState::Gaming), spawn_camera)
        .add_systems(PostUpdate, update_camera_target.before(update_camera));
    }
}
