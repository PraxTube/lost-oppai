use std::{
    f32::consts::{PI, TAU},
    time::Duration,
};

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_trickfilm::animation::AnimationPlayer2D;
use rand::{thread_rng, Rng};

use crate::{
    audio::PlaySound,
    player::{Player, PLAYER_SPAWN_POS},
    world::camera::{YSort, YSortChild},
    GameAssets, GameState,
};

use super::generation::BitMap;

const MAX_BIRD_COUNT: usize = 5;
const BIRD_SCALE: f32 = 1.0;

const JUMP_SPEED: f32 = 60.0;
const FLYING_SPEED: f32 = 160.0;

const MIN_PLAYER_DISTANCE: f32 = 40.0;
const PLAYER_JUMP_DISTANCE: f32 = 54.0;
const FLYING_AWAY_DISTANCE: f32 = 100.0;

const DESPAWN_DISTANCE: f32 = 400.0;
const SPAWN_DISTANCE: f32 = 210.0;
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
            YSort(-8.0),
            animator,
            SpriteSheetBundle {
                transform: Transform::from_translation(pos.extend(0.0))
                    .with_scale(Vec3::splat(BIRD_SCALE)),
                texture_atlas: assets.bird.clone(),
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

fn spawn_birds(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<(&Transform, &Player)>,
    q_birds: Query<With<Bird>>,
) {
    let (player_transform, player) = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if q_birds.iter().len() >= MAX_BIRD_COUNT {
        return;
    }

    let mut rng = thread_rng();

    let dir = player.average_direction().normalize_or_zero();
    if dir == Vec2::ZERO {
        return;
    }
    let offset =
        Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..0.0)) * RANDOM_OFFSET_DISTANCE;
    let pos = player_transform.translation.truncate() + dir * SPAWN_DISTANCE + offset;

    spawn_bird(
        &mut commands,
        &assets,
        pos,
        (pos - player_transform.translation.truncate()).normalize_or_zero(),
    );
}

fn despawn_birds(
    mut commands: Commands,
    q_player: Query<&Transform, (With<Player>, Without<Bird>)>,
    mut q_birds: Query<(Entity, &Transform), With<Bird>>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    for (entity, transform) in &mut q_birds {
        let dis = transform
            .translation
            .truncate()
            .distance_squared(player_transform.translation.truncate());

        if dis >= DESPAWN_DISTANCE.powi(2) {
            commands.entity(entity).despawn_recursive();
        }
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

fn pick_random_actions(time: Res<Time>, mut q_birds: Query<&mut Bird>) {
    let mut rng = thread_rng();

    for mut bird in &mut q_birds {
        if bird.state != BirdState::Idling {
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

fn move_birds(
    time: Res<Time>,
    mut q_birds: Query<(&mut Transform, &mut TextureAtlasSprite, &Bird)>,
) {
    for (mut transform, mut sprite, bird) in &mut q_birds {
        match bird.state {
            BirdState::Jumping => {
                transform.translation +=
                    bird.move_dir.extend(0.0) * JUMP_SPEED * time.delta_seconds();
                sprite.flip_x = bird.move_dir.x > 0.0;
            }
            BirdState::Flying => {
                transform.translation +=
                    bird.move_dir.extend(0.0) * FLYING_SPEED * time.delta_seconds();
                sprite.flip_x = bird.move_dir.x > 0.0;
            }
            _ => {}
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
            .normalize_or_zero();
        bird.move_dir = if dir == Vec2::ZERO { Vec2::X } else { dir };

        if dis < MIN_PLAYER_DISTANCE.powi(2) {
            bird.state = BirdState::Flying;
        } else if bird.state != BirdState::Jumping {
            bird.state = BirdState::Jumping;
        }
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
            })
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
                spawn_birds,
                despawn_birds,
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
        .add_systems(OnExit(GameState::AssetLoading), (spawn_initial_birds,));
    }
}
