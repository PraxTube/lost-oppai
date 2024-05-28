use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_trickfilm::animation::AnimationPlayer2D;
use rand::{thread_rng, Rng};

use crate::{
    player::Player,
    world::camera::{YSort, YSortChild},
    GameAssets, GameState,
};

use super::generation::BitMap;

const MAX_BIRD_COUNT: usize = 5;
const BIRD_SCALE: f32 = 1.0;

const JUMP_SPEED: f32 = 50.0;
const FLYING_SPEED: f32 = 120.0;

const MIN_PLAYER_DISTANCE: f32 = 48.0;
const FLY_EXCITEMENT_ADD: f32 = 0.5;
const JUMP_EXCITEMENT_ADD: f32 = 0.1;

#[derive(Component)]
struct Bird {
    state: BirdState,
    excitement: f32,
    action_timer: Timer,
    move_dir: Vec2,
}

impl Default for Bird {
    fn default() -> Self {
        Self {
            state: BirdState::default(),
            excitement: 1.0,
            action_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            move_dir: Vec2::ZERO,
        }
    }
}

#[derive(Default, PartialEq)]
enum BirdState {
    #[default]
    Idling,
    Jumping,
    Picking,
    Flying,
}

fn spawn_birds(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<&Transform, With<Player>>,
    q_birds: Query<With<Bird>>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if q_birds.iter().len() >= MAX_BIRD_COUNT {
        return;
    }

    let shadow = commands
        .spawn((
            YSortChild(-9.0),
            SpriteBundle {
                texture: assets.bird_shadow.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, -8.0, 0.0)),
                ..default()
            },
        ))
        .id();

    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.bird_animations[0].clone()).repeat();

    commands
        .spawn((
            Bird::default(),
            YSort(0.0),
            animator,
            SpriteSheetBundle {
                transform: Transform::from_translation(player_transform.translation)
                    .with_scale(Vec3::splat(BIRD_SCALE)),
                texture_atlas: assets.bird.clone(),
                ..default()
            },
        ))
        .push_children(&[shadow]);
}

fn return_to_idle_state(
    mut bitmap: ResMut<BitMap>,
    mut q_birds: Query<(&Transform, &AnimationPlayer2D, &mut Bird)>,
) {
    for (transform, player, mut bird) in &mut q_birds {
        if player.is_finished() {
            if bitmap.is_position_water(transform.translation.truncate()) {
                bird.state = BirdState::Flying;
            } else {
                bird.state = BirdState::Idling;
            }
        }
    }
}

fn update_bird_states(time: Res<Time>, mut q_birds: Query<&mut Bird>) {
    let mut rng = thread_rng();

    for mut bird in &mut q_birds {
        if bird.state != BirdState::Idling {
            continue;
        }

        bird.action_timer.tick(time.delta());
        if !bird.action_timer.just_finished() {
            continue;
        };

        bird.move_dir = Vec2::ZERO;
        let action_value = rng.gen_range(0.0..1.0);
        if action_value < 0.6 {
            bird.state = BirdState::Idling;
        } else if action_value < 0.7 {
            bird.state = BirdState::Picking;
        } else if action_value < 0.9 {
            bird.state = BirdState::Jumping;
            bird.move_dir = Vec2::from_angle(rng.gen_range(0.0..TAU));
        }
    }
}

fn move_birds(
    time: Res<Time>,
    mut q_birds: Query<(&mut Transform, &mut TextureAtlasSprite, &Bird)>,
) {
    for (mut transform, mut sprite, bird) in &mut q_birds {
        match bird.state {
            BirdState::Jumping => {
                transform.translation +=
                    bird.move_dir.extend(0.0) * bird.excitement * JUMP_SPEED * time.delta_seconds();
                sprite.flip_x = bird.move_dir.x > 0.0;
            }
            BirdState::Flying => {
                transform.translation += bird.move_dir.extend(0.0)
                    * bird.excitement
                    * FLYING_SPEED
                    * time.delta_seconds();
                sprite.flip_x = bird.move_dir.x > 0.0;
            }
            _ => {}
        };
    }
}

fn play_bird_animations(
    assets: Res<GameAssets>,
    mut q_birds: Query<(&mut AnimationPlayer2D, &Bird)>,
) {
    for (mut player, bird) in &mut q_birds {
        match bird.state {
            BirdState::Idling => player.play(assets.bird_animations[0].clone()).repeat(),
            BirdState::Jumping => player.play(assets.bird_animations[1].clone()),
            BirdState::Picking => player.play(assets.bird_animations[2].clone()),
            BirdState::Flying => player.play(assets.bird_animations[3].clone()).repeat(),
        };
    }
}

fn check_player_bird_distances(
    q_player: Query<&Transform, (With<Player>, Without<Bird>)>,
    mut q_birds: Query<(&Transform, &mut Bird)>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    let mut rng = thread_rng();

    for (transform, mut bird) in &mut q_birds {
        if bird.state != BirdState::Idling {
            return;
        }

        let dis = transform
            .translation
            .distance_squared(player_transform.translation);

        if dis > MIN_PLAYER_DISTANCE.powi(2) * bird.excitement {
            continue;
        }

        let action = rng.gen_range(0.0..1.0);

        bird.move_dir = (transform.translation - player_transform.translation)
            .truncate()
            .normalize_or_zero();
        if action < bird.excitement - 1.0 {
            bird.excitement += FLY_EXCITEMENT_ADD;
            bird.state = BirdState::Flying;
        } else {
            bird.excitement += JUMP_EXCITEMENT_ADD;
            bird.state = BirdState::Jumping;
        }
    }
}

fn update_excitement_meter(time: Res<Time>, mut q_birds: Query<&mut Bird>) {
    for mut bird in &mut q_birds {
        if bird.state == BirdState::Jumping || bird.state == BirdState::Flying {
            continue;
        }
        bird.excitement = (bird.excitement - time.delta_seconds()).clamp(1.0, 2.0);
    }
}

pub struct FaunaPlugin;

impl Plugin for FaunaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_birds,
                update_bird_states,
                move_birds,
                play_bird_animations,
                check_player_bird_distances,
                update_excitement_meter,
                return_to_idle_state,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
