use std::f32::consts::PI;

use rand::{thread_rng, Rng};

use bevy::{prelude::*, utils::HashSet};
use bevy_particle_systems::{
    CircleSegment, ColorOverTime, Curve, CurvePoint, EmitterShape, JitteredValue, Noise2D,
    ParticleSystem, ParticleSystemBundle, Playing, VectorOverTime, VelocityModifier,
};
use bevy_rapier2d::prelude::*;

use crate::{
    npc::Npc,
    ui::keyboard_hint::{KeyboardHint, KEYBOARD_ICON_RADIUS},
    world::camera::{YSort, YSortStatic, YSortStaticChild},
    GameAssets, GameState,
};

use super::{
    chunk_manager::{DespawnedChunk, SpawnedChunk},
    generation::BitMap,
    poisson_sampling::generate_poisson_points_variable_radii,
    CHUNK_SIZE, TILE_SIZE,
};

const ROCKS_COUNT: usize = 3;

const REJECTION_ITER: usize = 20;
const MIN_RADIUS: f32 = 1.0;
const MAX_RADIUS: f32 = 4.0;

const TREE_RADIUS: f32 = 3.0;
const BUSH_1_RADIUS: f32 = 2.25;
const BUSH_2_RADIUS: f32 = 1.25;
const ROCK_RADIUS: f32 = 1.0;

const NPC_FLORA_RADIUS: f32 = 64.0;

#[derive(Component)]
struct SakuraPedal;
#[derive(Component)]
pub struct TreeCollider;

#[derive(Component)]
struct Flora {
    chunk_pos: IVec2,
}

impl Flora {
    fn new(chunk_pos: IVec2) -> Self {
        Self { chunk_pos }
    }
}

fn spawn_rock(commands: &mut Commands, assets: &Res<GameAssets>, chunk_pos: IVec2, pos: Vec3) {
    let collider = commands
        .spawn((
            Collider::cuboid(8.0, 8.0),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))),
        ))
        .id();

    let mut rng = thread_rng();

    commands
        .spawn((
            Flora::new(chunk_pos),
            YSort(-40.0),
            SpriteBundle {
                texture: assets.rocks_texture.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            },
            TextureAtlas {
                layout: assets.rocks_layout.clone(),
                index: rng.gen_range(0..ROCKS_COUNT),
            },
        ))
        .push_children(&[collider]);
}

fn spawn_bush(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    chunk_pos: IVec2,
    pos: Vec3,
    index: usize,
) {
    let texture = if index == 0 {
        assets.bush1.clone()
    } else {
        assets.bush2.clone()
    };
    let c = if index == 0 {
        Collider::cuboid(16.0, 8.0)
    } else {
        Collider::cuboid(8.0, 8.0)
    };

    let collider = commands
        .spawn((
            c,
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, -8.0, 0.0))),
        ))
        .id();

    commands
        .spawn((
            Flora::new(chunk_pos),
            YSort(0.0),
            SpriteBundle {
                transform: Transform::from_translation(pos),
                texture,
                ..default()
            },
        ))
        .push_children(&[collider]);
}

fn spawn_tree(commands: &mut Commands, assets: &Res<GameAssets>, chunk_pos: IVec2, pos: Vec3) {
    let pos = pos + Vec3::new(0.0, 48.0, 0.0);

    let trunk = commands
        .spawn((
            YSortStaticChild(16.1),
            SpriteBundle {
                texture: assets.tree_trunk.clone(),
                ..default()
            },
        ))
        .id();

    let shadow = commands
        .spawn((
            YSortStaticChild(64.0),
            SpriteBundle {
                texture: assets.tree_shadow.clone(),
                ..default()
            },
        ))
        .id();

    let collider = commands
        .spawn((
            TreeCollider,
            Collider::cuboid(16.0, 8.0),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -48.0, 0.0,
            ))),
        ))
        .id();

    let tree_pedals = commands
        .spawn((
            SakuraPedal,
            Playing,
            ParticleSystemBundle {
                particle_system: ParticleSystem {
                    max_particles: 0,
                    texture: assets.tree_pedal.clone().into(),
                    spawn_rate_per_second: 2.0.into(),
                    initial_speed: JitteredValue::jittered(50.0, -3.0..3.0),
                    lifetime: JitteredValue::jittered(1.0, -0.5..1.5),
                    color: ColorOverTime::Gradient(Curve::new(vec![
                        CurvePoint::new(Color::srgba(1.0, 1.0, 1.0, 0.0), 0.0),
                        CurvePoint::new(Color::WHITE, 0.3),
                        CurvePoint::new(Color::WHITE, 0.8),
                        CurvePoint::new(Color::srgba(1.0, 1.0, 1.0, 0.0), 0.95),
                        CurvePoint::new(Color::srgba(1.0, 1.0, 1.0, 0.0), 1.0),
                    ])),
                    emitter_shape: EmitterShape::CircleSegment(CircleSegment {
                        opening_angle: PI / 2.0,
                        direction_angle: 3.0 / 2.0 * PI,
                        radius: 0.0.into(),
                    }),
                    initial_rotation: JitteredValue::jittered(PI, -PI..PI),
                    rotation_speed: JitteredValue::jittered(PI / 2.0, -PI / 4.0..PI / 4.0),
                    velocity_modifiers: vec![
                        VelocityModifier::Vector(VectorOverTime::Constant(
                            7.0 * Vec3::new(3.0, -1.5, 0.0),
                        )),
                        VelocityModifier::Noise(Noise2D::new(0.1, 2.0, Vec2::ZERO)),
                    ],
                    scale: 0.5.into(),
                    z_value_override: Some(100.0.into()),
                    looping: true,
                    despawn_particles_with_system: true,
                    ..ParticleSystem::default()
                },
                ..ParticleSystemBundle::default()
            },
        ))
        .id();

    commands
        .spawn((
            Flora::new(chunk_pos),
            YSortStatic(40.0),
            SpriteBundle {
                transform: Transform::from_translation(pos),
                texture: assets.tree.clone(),
                ..default()
            },
        ))
        .push_children(&[trunk, shadow, collider, tree_pedals]);
}

fn spawn_flora(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    bitmap: &mut ResMut<BitMap>,
    chunk_pos: IVec2,
    ipos: IVec2,
    radius: f32,
) {
    let v = IVec2::new(
        ipos.x + chunk_pos.x * CHUNK_SIZE as i32,
        ipos.y + chunk_pos.y * CHUNK_SIZE as i32,
    );
    let pos = TILE_SIZE * Vec3::new(v.x as f32, v.y as f32, 0.0);

    if v.x.unsigned_abs() % CHUNK_SIZE == 0 || v.y.unsigned_abs() % CHUNK_SIZE == 0 {
        return;
    }
    if radius > TREE_RADIUS {
        if !(bitmap.get_flora_flag(v)
            && bitmap.get_flora_flag(v + IVec2::new(1, 0))
            && bitmap.get_flora_flag(v + IVec2::new(0, 1))
            && bitmap.get_flora_flag(v + IVec2::new(1, 1))
            && bitmap.get_flora_flag(v + IVec2::new(2, 0))
            && bitmap.get_flora_flag(v + IVec2::new(-2, 0)))
        {
            return;
        }
        spawn_tree(commands, assets, chunk_pos, pos);
    } else if radius > BUSH_1_RADIUS {
        if !(bitmap.get_flora_flag(v)
            && bitmap.get_flora_flag(v - IVec2::Y)
            && bitmap.get_flora_flag(v + IVec2::X)
            && bitmap.get_flora_flag(v - IVec2::X))
        {
            return;
        }
        spawn_bush(commands, assets, chunk_pos, pos, 0);
    } else if radius > BUSH_2_RADIUS {
        if !(bitmap.get_flora_flag(v) && bitmap.get_flora_flag(v - IVec2::Y)) {
            return;
        }
        spawn_bush(commands, assets, chunk_pos, pos, 1);
    } else if radius > ROCK_RADIUS {
        if !bitmap.get_flora_flag(v) {
            return;
        }
        spawn_rock(commands, assets, chunk_pos, pos);
    }
}

fn spawn_flora_chunks(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut bitmap: ResMut<BitMap>,
    mut ev_spawned_chunk: EventReader<SpawnedChunk>,
) {
    for ev in ev_spawned_chunk.read() {
        let seed =
            bitmap.seed() as u64 + ev.pos.x.unsigned_abs() as u64 + ev.pos.y.unsigned_abs() as u64;

        // Because we discretize our positions here, we have to make sure that no two
        // flora positions map to the same IVec2.
        // We filter out any duplicates with a HashSet.
        let mut unique_points = HashSet::new();
        let points_with_radius: Vec<(IVec2, f32)> = generate_poisson_points_variable_radii(
            MIN_RADIUS,
            MAX_RADIUS,
            CHUNK_SIZE as f32 * Vec2::ONE,
            REJECTION_ITER,
            seed,
        )
        .into_iter()
        .map(|p_r| {
            let v = IVec2::new(p_r.x.floor() as i32, p_r.y.floor() as i32);
            (v, p_r.z)
        })
        .filter(|(p, _)| unique_points.insert(*p))
        .collect();

        for (p, r) in points_with_radius {
            spawn_flora(&mut commands, &assets, &mut bitmap, ev.pos, p, r);
        }
    }
}

fn despawn_flora_chunks(
    mut commands: Commands,
    q_floras: Query<(Entity, &Flora)>,
    mut ev_despawned_chunk: EventReader<DespawnedChunk>,
) {
    for ev in ev_despawned_chunk.read() {
        for (entity, flora) in &q_floras {
            if flora.chunk_pos == ev.chunk_pos {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn despawn_flora_around_start_hint(
    mut commands: Commands,
    q_keyboard_icon: Query<&Transform, (With<KeyboardHint>, Without<Flora>)>,
    q_floras: Query<(Entity, &Transform), Added<Flora>>,
) {
    let start_hint_transform = match q_keyboard_icon.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    for (entity, flora_transform) in &q_floras {
        if flora_transform
            .translation
            .distance_squared(start_hint_transform.translation)
            <= KEYBOARD_ICON_RADIUS.powi(2)
        {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_flora_around_npcs(
    mut commands: Commands,
    q_npcs: Query<&Transform, (With<Npc>, Without<Flora>)>,
    q_floras: Query<(Entity, &Transform), Added<Flora>>,
) {
    for (entity, flora_transform) in &q_floras {
        for npc_transform in &q_npcs {
            if flora_transform
                .translation
                .distance_squared(npc_transform.translation)
                <= NPC_FLORA_RADIUS.powi(2)
            {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn play_sakura_pedal_particles(mut q_pedals: Query<&mut ParticleSystem, Added<SakuraPedal>>) {
    for mut system in &mut q_pedals {
        system.max_particles = 100;
    }
}

pub struct FloraPlugin;

impl Plugin for FloraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                play_sakura_pedal_particles.before(spawn_flora_chunks),
                spawn_flora_chunks,
                despawn_flora_chunks,
                despawn_flora_around_start_hint,
                despawn_flora_around_npcs,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
