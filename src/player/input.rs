use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::input::InputSystem;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};

use crate::world::MainCamera;
use crate::GameState;

#[derive(Resource, Default)]
pub struct MouseWorldCoords(pub Vec2);

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub move_direction: Vec2,
    pub scroll: f32,
    pub running: bool,
    pub escape: bool,

    pub start_dialogue: bool,
    pub dialogue_confirm: bool,
    pub dialogue_continue: bool,
    pub dialogue_direction: i8,

    pub toggle_fullscreen: bool,
    pub toggle_debug: bool,
}

fn reset_player_input(mut player_input: ResMut<PlayerInput>) {
    *player_input = PlayerInput::default();
}

pub fn fetch_mouse_world_coords(
    mut mouse_coords: ResMut<MouseWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = match q_camera.get_single() {
        Ok(c) => (c.0, c.1),
        Err(_) => return,
    };
    let window = match q_window.get_single() {
        Ok(w) => w,
        Err(_) => return,
    };

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_coords.0 = world_position;
    }
}

fn fetch_scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut player_input: ResMut<PlayerInput>,
) {
    for ev in scroll_evr.read() {
        let scroll = match ev.unit {
            MouseScrollUnit::Line => {
                if ev.y > 0.0 {
                    -1.0
                } else {
                    1.0
                }
            }
            MouseScrollUnit::Pixel => {
                if ev.y > 0.0 {
                    -1.0
                } else {
                    1.0
                }
            }
        };
        player_input.scroll = scroll;
    }
}

fn input_scroll(keys: Res<Input<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    let mut zoom = 0.0;
    if keys.just_pressed(KeyCode::Plus) {
        zoom -= 1.0;
    }
    if keys.just_pressed(KeyCode::Minus) {
        zoom += 1.0;
    }

    if zoom != 0.0 {
        player_input.scroll = zoom;
    }
}

fn player_movement(keys: Res<Input<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    let mut direction = Vec2::default();

    if keys.pressed(KeyCode::J) || keys.pressed(KeyCode::S) {
        direction += Vec2::new(0.0, -1.0);
    }
    if keys.pressed(KeyCode::K) || keys.pressed(KeyCode::W) {
        direction += Vec2::new(0.0, 1.0);
    }
    if keys.pressed(KeyCode::F) || keys.pressed(KeyCode::D) {
        direction += Vec2::new(1.0, 0.0);
    }
    if keys.pressed(KeyCode::A) {
        direction += Vec2::new(-1.0, 0.0);
    }

    player_input.move_direction = direction.normalize_or_zero();
}

fn input_running(keys: Res<Input<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    player_input.running = keys.pressed(KeyCode::ShiftLeft);
}

fn input_escape(keys: Res<Input<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    player_input.escape = keys.just_pressed(KeyCode::Escape);
}

fn input_dialogue(
    keys: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut player_input: ResMut<PlayerInput>,
) {
    let mut direction = 0;

    if keys.just_pressed(KeyCode::J)
        || keys.just_pressed(KeyCode::S)
        || keys.just_pressed(KeyCode::Down)
    {
        direction -= 1;
    }
    if keys.just_pressed(KeyCode::K)
        || keys.just_pressed(KeyCode::W)
        || keys.just_pressed(KeyCode::Up)
    {
        direction += 1;
    }
    player_input.dialogue_direction = direction;

    player_input.start_dialogue = keys.just_pressed(KeyCode::E);
    player_input.dialogue_confirm = keys.just_pressed(KeyCode::Return);

    player_input.dialogue_continue =
        keys.just_pressed(KeyCode::Space) || mouse_buttons.just_pressed(MouseButton::Left);
}

fn toggle_fullscreen(keys: Res<Input<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    player_input.toggle_fullscreen = keys.just_pressed(KeyCode::B);
}

fn toggle_debug(keys: Res<Input<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    player_input.toggle_debug = keys.just_pressed(KeyCode::F3);
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                fetch_scroll_events,
                fetch_mouse_world_coords,
                input_scroll,
                player_movement,
                input_running,
                input_escape,
                input_dialogue,
                toggle_fullscreen,
                toggle_debug,
            )
                .run_if(in_state(GameState::Gaming))
                .after(InputSystem),
        )
        .init_resource::<PlayerInput>()
        .init_resource::<MouseWorldCoords>()
        .add_systems(PreUpdate, reset_player_input.before(InputSystem));
    }
}
