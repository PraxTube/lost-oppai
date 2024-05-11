use rand::{Rng, SeedableRng};

use bevy::{prelude::*, utils::HashSet};
use noisy_bevy::simplex_noise_2d_seeded;

use crate::{world::map::poisson_sampling::generate_poisson_points, GameRng, GameState};

use super::{BitMap, GRASS_TYPE_INDEX, PATH_TYPE_INDEX};

const NOISE_ZOOM: f32 = 0.02;
const SAMPLE_RATE: f32 = 3.0;
const START_FILL_RADIUS: i32 = 15;

const MIN_RADIUS: i32 = 1;
const MAX_RADIUS: i32 = MIN_RADIUS + 1;
const MIN_RADIUS_GRASS: i32 = MAX_RADIUS + 1;
const MAX_RADIUS_GRASS: i32 = MIN_RADIUS_GRASS + 3;

const DISK_RADIUS: f32 = 35.0;
const REGION_SIZE: Vec2 = Vec2::new(150.0, 150.0);
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
                bitmap.set_type_index(v + offset, PATH_TYPE_INDEX);
            } else if dis < sqrt_radius_grass && !bitmap.get_path_flag(v + offset) {
                bitmap.set_type_index(v + offset, GRASS_TYPE_INDEX);
            }
        }
    }
}

fn fill_path_points(bitmap: &mut ResMut<BitMap>, points: Vec<IVec2>) {
    for v in points {
        fill_path_point(bitmap, v);
    }
}

fn calculate_points(index: usize, points: &Vec<Vec2>) -> (Vec2, Vec2) {
    let mut min_dis = f32::INFINITY;
    let mut closest_index = 0;

    for i in 0..points.len() {
        if i == index {
            continue;
        }

        let dis = points[index].distance_squared(points[i]);
        if dis < min_dis {
            closest_index = i;
            min_dis = dis;
        }
    }
    (points[index], points[closest_index])
}

fn generate_edges(vertices: &Vec<Vec2>) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for i in 0..vertices.len() {
        for j in 0..vertices.len() {
            if i >= j {
                continue;
            }
            edges.push((i, j, vertices[i].distance_squared(vertices[j])));
        }
    }

    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    edges.into_iter().map(|(a, b, _)| (a, b)).collect()
}

fn kruskals_edges(n: usize, sorted_edges: Vec<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();
    let mut sets = Vec::new();
    for i in 0..n {
        let mut s = HashSet::new();
        s.insert(i);
        sets.push(s);
    }

    for (u, v) in sorted_edges {
        let mut u_set_index = None;
        let mut v_set_index = None;

        for (index, set) in sets.iter().enumerate() {
            if set.contains(&u) {
                u_set_index = Some(index);
            }
            if set.contains(&v) {
                v_set_index = Some(index);
            }
            if u_set_index.is_some() && v_set_index.is_some() {
                break;
            }
        }

        if let (Some(u_index), Some(v_index)) = (u_set_index, v_set_index) {
            if u_index != v_index {
                result.insert((u, v));
                let other = sets[v_index].clone();
                sets[u_index].extend(other);
                sets.remove(v_index);
            }
        }
    }
    result
}

fn generate_path(mut bitmap: ResMut<BitMap>) {
    let points = generate_poisson_points(
        DISK_RADIUS,
        REGION_SIZE,
        POISSON_REJECTION_ITER,
        bitmap.seed() as u64,
    );
    bitmap.append_hotspots(&points);

    let mut rng = GameRng::seed_from_u64(bitmap.seed() as u64);

    let sample_size = (DISK_RADIUS * SAMPLE_RATE) as usize;

    let edges = kruskals_edges(points.len(), generate_edges(&points));
    for (u, v) in edges {
        let (p1, p2) = (points[u], points[v]);
        let (c1, c2) = generate_bezier_points(&mut rng, p1, p2);
        let points = compute_path_points(p1, p2, c1, c2, sample_size);
        fill_path_points(&mut bitmap, points);
    }
}

fn fill_player_starting_position(mut bitmap: ResMut<BitMap>) {
    for x in -START_FILL_RADIUS..=START_FILL_RADIUS {
        for y in -START_FILL_RADIUS..=START_FILL_RADIUS {
            let v = IVec2::new(x, y);
            if v.length_squared() <= START_FILL_RADIUS.pow(2) {
                bitmap.set_type_index(v, GRASS_TYPE_INDEX);
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
