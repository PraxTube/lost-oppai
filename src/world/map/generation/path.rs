use rand::{Rng, SeedableRng};

use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d_seeded;

use crate::{world::map::poisson_sampling::generate_poisson_points, GameRng, GameState};

use super::{
    graph::{connect_outer_vertices, kruskals_edges},
    BitMap, GRASS_TYPE_MASK, PATH_TYPE_MASK,
};

const NOISE_ZOOM: f32 = 0.02;
const START_FILL_RADIUS: i32 = 15;

const MIN_RADIUS: i32 = 1;
const MAX_RADIUS: i32 = MIN_RADIUS + 1;
const MIN_RADIUS_GRASS: i32 = MAX_RADIUS + 1;
const MAX_RADIUS_GRASS: i32 = MIN_RADIUS_GRASS + 3;

const DISK_RADIUS: f32 = 35.0;
const SAMPLE_RATE: usize = 3 * DISK_RADIUS as usize;
const REGION_SIZE: Vec2 = Vec2::new(120.0, 120.0);
const POISSON_REJECTION_ITER: usize = 20;

// Use bezier curve to compute the points along
// the curve based on the given sample_size.
fn compute_path_points(p1: Vec2, p2: Vec2, c1: Vec2, c2: Vec2, sample_size: usize) -> Vec<IVec2> {
    fn curve(p1: Vec2, p2: Vec2, c1: Vec2, c2: Vec2, t: f32) -> Vec2 {
        (1.0 - t).powi(3) * p1
            + 3.0 * t * (1.0 - t).powi(2) * c1
            + 3.0 * t.powi(2) * (1.0 - t) * c2
            + t.powi(3) * p2
    }

    let mut points = Vec::new();
    for i in 0..sample_size {
        let t = i as f32 / sample_size as f32;
        let pos = curve(p1, p2, c1, c2, t);
        let dis_pos = IVec2::new(pos.x as i32, pos.y as i32);
        points.push(dis_pos);
    }
    points
}

fn generate_bezier_points(rng: &mut GameRng, p1: Vec2, p2: Vec2) -> (Vec2, Vec2) {
    let min_p = Vec2::new(p1.x.min(p2.x), p1.y.min(p2.y));
    let max_p = Vec2::new(p1.x.max(p2.x), p1.y.max(p2.y));
    let c1 = Vec2::new(
        rng.gen_range(min_p.x..max_p.x),
        rng.gen_range(min_p.y..max_p.y),
    );
    let c2 = Vec2::new(
        rng.gen_range(min_p.x..max_p.x),
        rng.gen_range(min_p.y..max_p.y),
    );
    (c1, c2)
}

fn fill_path_point(bitmap: &mut ResMut<BitMap>, v: IVec2) {
    let w = Vec2::new(v.x as f32, v.y as f32);

    let noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, bitmap.seed());
    let secondary_noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, bitmap.seed() + 1.0);
    let radius = (MIN_RADIUS as f32
        + 0.25 * (noise + secondary_noise + 2.0) * (MAX_RADIUS - MIN_RADIUS) as f32)
        as i32;
    let sqrt_radius = radius.pow(2);
    let radius_grass = (MIN_RADIUS_GRASS as f32
        + 0.25 * (noise + secondary_noise + 2.0) * (MAX_RADIUS_GRASS - MIN_RADIUS_GRASS) as f32)
        as i32;
    let sqrt_radius_grass = radius_grass.pow(2);

    for x in -radius_grass..=radius_grass {
        for y in -radius_grass..=radius_grass {
            let offset = IVec2::new(x, y);
            let dis = offset.length_squared();
            if dis < sqrt_radius {
                bitmap.set_type_index(v + offset, PATH_TYPE_MASK);
            } else if dis < sqrt_radius_grass && !bitmap.get_path_flag(v + offset) {
                bitmap.set_type_index(v + offset, GRASS_TYPE_MASK);
            }
        }
    }
}

fn fill_path_points(bitmap: &mut ResMut<BitMap>, points: Vec<IVec2>) {
    for v in points {
        fill_path_point(bitmap, v);
    }
}

fn generate_path(mut bitmap: ResMut<BitMap>) {
    let vertices = generate_poisson_points(
        DISK_RADIUS,
        REGION_SIZE,
        POISSON_REJECTION_ITER,
        bitmap.seed() as u64,
    );
    bitmap.set_vertices(&vertices);

    let mut edges = kruskals_edges(&vertices);
    connect_outer_vertices(&vertices, &mut edges);
    bitmap.set_edges(&edges);
    let mut edges: Vec<(usize, usize)> = edges.into_iter().collect();
    edges.sort();

    let mut rng = GameRng::seed_from_u64(bitmap.seed() as u64);
    for (u, v) in edges {
        let (p1, p2) = (vertices[u], vertices[v]);
        let (c1, c2) = generate_bezier_points(&mut rng, p1, p2);
        let points = compute_path_points(p1, p2, c1, c2, SAMPLE_RATE);
        fill_path_points(&mut bitmap, points);
    }
}

fn fill_player_starting_position(mut bitmap: ResMut<BitMap>) {
    for x in -START_FILL_RADIUS..=START_FILL_RADIUS {
        for y in -START_FILL_RADIUS..=START_FILL_RADIUS {
            let v = IVec2::new(x, y);
            if v.length_squared() <= START_FILL_RADIUS.pow(2) {
                bitmap.set_type_index(v, GRASS_TYPE_MASK);
            }
        }
    }
}

pub struct PathGenerationPlugin;

impl Plugin for PathGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::AssetLoading),
            (fill_player_starting_position, generate_path).chain(),
        );
    }
}
