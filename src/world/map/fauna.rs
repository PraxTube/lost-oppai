use std::{
    f32::consts::{PI, TAU},
    time::Duration,
};

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::animation::AnimationPlayer2D;
use rand::{thread_rng, Rng};

use crate::{
    audio::PlaySound,
    player::{Player, PLAYER_SPAWN_POS},
    world::camera::{YSort, YSortChild},
    GameAssets, GameState,
};

use super::{flora::TreeCollider, generation::BitMap};

const MAX_BIRD_COUNT: usize = 10000;
const BIRD_SCALE: f32 = 1.0;

const JUMP_SPEED: f32 = 60.0;
const FLYING_SPEED: f32 = 160.0;

const MIN_PLAYER_DISTANCE: f32 = 40.0;
const PLAYER_JUMP_DISTANCE: f32 = 54.0;
const FLYING_AWAY_DISTANCE: f32 = 100.0;

const DESPAWN_DISTANCE: f32 = 400.0;
const SPAWN_DISTANCE: f32 = 250.0;
const RANDOM_OFFSET_DISTANCE: f32 = 20.0;

#[derive(Component)]
struct Bird {
    state: BirdState,
    old_state: BirdState,
    action_timer: Timer,
    move_dir: Vec2,
    flapping_sound_container: Entity,
}

impl Bird {
    fn new(move_dir: Vec2, flapping_sound_container: Entity) -> Self {
        Self {
            state: BirdState::default(),
            old_state: BirdState::default(),
            action_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            move_dir,
            flapping_sound_container,
        }
    }
}

#[derive(Default, PartialEq, Debug, Clone, Copy)]
enum BirdState {
    #[default]
    Idling,
    Jumping,
    Picking,
    Flying,
}

fn spawn_bird(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec2, move_dir: Vec2) {
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

    let flapping_sound_container = commands.spawn(SpatialBundle::default()).id();

    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.bird_animations[0].clone()).repeat();

    commands
        .spawn((
            Bird::new(move_dir, flapping_sound_container),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            YSort(-8.0),
            animator,
            SpriteBundle {
                texture: assets.bird_texture.clone(),
                transform: Transform::from_translation(pos.extend(0.0))
                    .with_scale(Vec3::splat(BIRD_SCALE)),
                ..default()
            },
            TextureAtlas {
                layout: assets.bird_layout.clone(),
                ..default()
            },
        ))
        .push_children(&[shadow, flapping_sound_container]);
}

fn spawn_initial_birds(mut commands: Commands, assets: Res<GameAssets>) {
    let mut rng = thread_rng();
    for _ in 0..MAX_BIRD_COUNT {
        let dir = Vec2::from_angle(rng.gen_range(0.0..2.0 * PI));
        let pos = PLAYER_SPAWN_POS.truncate() + dir.normalize_or_zero() * SPAWN_DISTANCE;
        spawn_bird(&mut commands, &assets, pos, Vec2::NEG_Y);
    }
}

fn teleport_birds(
    q_player: Query<(&Transform, &Player), Without<Bird>>,
    mut q_birds: Query<&mut Transform, With<Bird>>,
) {
    let (player_transform, player) = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    for mut transform in &mut q_birds {
        let dis = transform
            .translation
            .truncate()
            .distance_squared(player_transform.translation.truncate());

        if dis < DESPAWN_DISTANCE.powi(2) {
            continue;
        }

        let mut rng = thread_rng();

        let dir = player.average_direction().normalize_or_zero();
        if dir == Vec2::ZERO {
            return;
        }
        let offset =
            Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..0.0)) * RANDOM_OFFSET_DISTANCE;
        transform.translation =
            (player_transform.translation.truncate() + dir * SPAWN_DISTANCE + offset).extend(0.0);
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

fn pick_random_actions(
    time: Res<Time>,
    mut bitmap: ResMut<BitMap>,
    mut q_birds: Query<(&Transform, &mut Bird)>,
) {
    let mut rng = thread_rng();

    for (transform, mut bird) in &mut q_birds {
        if bird.state != BirdState::Idling {
            continue;
        }

        if bitmap.is_position_water(transform.translation.truncate()) {
            bird.state = BirdState::Flying;
            bird.move_dir = Vec2::from_angle(rng.gen_range(0.0..TAU));
            continue;
        }

        bird.action_timer.tick(time.delta());
        if !bird.action_timer.just_finished() {
            continue;
        };

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

fn return_to_idle_state(
    mut bitmap: ResMut<BitMap>,
    q_player: Query<&Transform, (With<Player>, Without<Bird>)>,
    mut q_birds: Query<(&Transform, &AnimationPlayer2D, &mut Bird)>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    for (transform, player, mut bird) in &mut q_birds {
        if bird.state == BirdState::Flying {
            if transform
                .translation
                .truncate()
                .distance_squared(player_transform.translation.truncate())
                <= FLYING_AWAY_DISTANCE.powi(2)
            {
                continue;
            }
        } else if !player.is_finished() {
            continue;
        }

        if bitmap.is_position_water(transform.translation.truncate()) {
            bird.state = BirdState::Flying;
        } else {
            bird.state = BirdState::Idling;
        }
    }
}

fn move_birds(mut q_birds: Query<(&mut Velocity, &mut Sprite, &Bird)>) {
    for (mut velocity, mut sprite, bird) in &mut q_birds {
        let speed = match bird.state {
            BirdState::Jumping => JUMP_SPEED,
            BirdState::Flying => FLYING_SPEED,
            _ => 0.0,
        };

        velocity.linvel = bird.move_dir * speed;
        if bird.move_dir.x != 0.0 {
            sprite.flip_x = bird.move_dir.x > 0.0;
        }
    }
}

fn check_player_bird_distances(
    rapier_context: Res<RapierContext>,
    q_player: Query<&Transform, (With<Player>, Without<Bird>)>,
    q_tree_colliders: Query<&TreeCollider>,
    mut q_birds: Query<(&Transform, &mut Bird)>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    for (transform, mut bird) in &mut q_birds {
        if bird.state == BirdState::Flying {
            continue;
        }

        let dis = transform
            .translation
            .truncate()
            .distance_squared(player_transform.translation.truncate());

        if dis > PLAYER_JUMP_DISTANCE.powi(2) {
            continue;
        }

        let dir = (transform.translation - player_transform.translation)
            .truncate()
            .normalize_or(Vec2::X);

        let (state, max_toi) = if dis < MIN_PLAYER_DISTANCE.powi(2) {
            (BirdState::Flying, FLYING_SPEED * 2.0)
        } else {
            (BirdState::Jumping, JUMP_SPEED * 1.0)
        };
        bird.state = state;

        let ray_pos = transform.translation.truncate();
        let filter = QueryFilter::default();
        let mut biggest_toi = 0.0;
        let mut final_dir = dir;

        for dir in [
            dir,
            Vec2::from_angle(dir.to_angle() + PI / 4.0),
            Vec2::from_angle(dir.to_angle() - PI / 4.0),
        ] {
            if let Some((entity, toi)) =
                rapier_context.cast_ray(ray_pos, dir, max_toi, true, filter)
            {
                if q_tree_colliders.get(entity).is_ok() {
                    if toi > biggest_toi {
                        biggest_toi = toi;
                        final_dir = dir;
                    }
                }
            } else {
                biggest_toi = f32::MAX;
                final_dir = dir;
            }
        }
        bird.move_dir = final_dir;
    }
}

fn play_bird_step_sounds(
    assets: Res<GameAssets>,
    q_birds: Query<(Entity, &Bird)>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    for (entity, bird) in &q_birds {
        if bird.state == BirdState::Jumping && bird.old_state != BirdState::Jumping {
            ev_play_sound.send(PlaySound {
                clip: assets.bird_step_sound.clone(),
                volume: 3.0,
                rand_speed_intensity: 0.2,
                parent: Some(entity),
                ..default()
            });
        }
    }
}

fn play_bird_flap_sounds(
    assets: Res<GameAssets>,
    q_birds: Query<&Bird>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    for bird in &q_birds {
        if bird.state == BirdState::Flying && bird.old_state != BirdState::Flying {
            ev_play_sound.send(PlaySound {
                clip: assets.bird_flaps_sound.clone(),
                repeat: true,
                parent: Some(bird.flapping_sound_container),
                volume: 2.5,
                rand_speed_intensity: 0.2,
                ..default()
            });
        }
    }
}

fn stop_bird_flap_sounds(
    mut instances: ResMut<Assets<AudioInstance>>,
    q_emitters: Query<(&Parent, &AudioEmitter)>,
    q_birds: Query<&Bird>,
) {
    for bird in &q_birds {
        if bird.state != BirdState::Flying && bird.old_state == BirdState::Flying {
            for (parent, emitter) in &q_emitters {
                if parent.get() == bird.flapping_sound_container {
                    emitter.instances.iter().for_each(|instance_handle| {
                        if let Some(instance) = instances.get_mut(instance_handle) {
                            instance.stop(AudioTween::linear(Duration::from_millis(600)));
                        }
                    });
                }
            }
        }
    }
}

fn update_old_bird_state(mut q_birds: Query<&mut Bird>) {
    for mut bird in &mut q_birds {
        bird.old_state = bird.state;
    }
}

pub struct FaunaPlugin;

impl Plugin for FaunaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                teleport_birds,
                (
                    check_player_bird_distances,
                    pick_random_actions,
                    return_to_idle_state,
                    play_bird_animations,
                    move_birds,
                    play_bird_step_sounds,
                    play_bird_flap_sounds,
                    stop_bird_flap_sounds,
                    update_old_bird_state,
                )
                    .chain(),
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_systems(OnEnter(GameState::Gaming), (spawn_initial_birds,));
    }
}
