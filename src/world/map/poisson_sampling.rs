use std::f32::consts::PI;

use rand::{Rng, SeedableRng};

use bevy::prelude::*;

use crate::GameRng;

pub fn generate_poisson_points(
    radius: f32,
    region_size: Vec2,
    rejection_iter: usize,
    seed: u64,
) -> Vec<Vec2> {
    let cell_size = radius / f32::sqrt(2.0);
    let grid_size_x = (region_size.x / cell_size).floor() as usize + 1;
    let grid_size_y = (region_size.y / cell_size).floor() as usize + 1;
    let mut grid: Vec<Vec<i32>> = vec![vec![-1; grid_size_x]; grid_size_y];

    let mut points: Vec<Vec2> = vec![region_size / 2.0];
    let mut spawn_points: Vec<Vec2> = vec![region_size / 2.0];

    let mut rng = GameRng::seed_from_u64(seed);

    while !spawn_points.is_empty() {
        let spawn_index = rng.gen_range(0..spawn_points.len());
        let spawn_center = spawn_points[spawn_index];

        let mut candidate_accepted = false;
        for _ in 0..rejection_iter {
            let angle: f32 = rng.gen_range(0.0..2.0 * PI);
            let dir = Vec2::new(angle.sin(), angle.cos());
            let candidate = spawn_center + dir * rng.gen_range(radius..2.0 * radius);

            if !is_valid(candidate, region_size, cell_size, radius, &points, &grid) {
                continue;
            }

            points.push(candidate);
            spawn_points.push(candidate);
            grid[(candidate.y / cell_size).floor() as usize]
                [(candidate.x / cell_size).floor() as usize] = points.len() as i32 - 1;
            candidate_accepted = true;
            break;
        }

        if !candidate_accepted {
            spawn_points.remove(spawn_index);
        }
    }

    let points = points.iter_mut().map(|x| *x - region_size / 2.0).collect();
    points
}

pub fn generate_poisson_points_variable_radii(
    min_radius: f32,
    max_radius: f32,
    region_size: Vec2,
    rejection_iter: usize,
    seed: u64,
) -> Vec<Vec3> {
    let cell_size = max_radius / f32::sqrt(2.0);
    let grid_size_x = (region_size.x / cell_size).floor() as usize + 1;
    let grid_size_y = (region_size.y / cell_size).floor() as usize + 1;
    let mut grid: Vec<Vec<i32>> = vec![vec![-1; grid_size_x]; grid_size_y];

    let mut points: Vec<Vec3> = Vec::new();
    let mut spawn_points: Vec<Vec2> = vec![region_size / 2.0];

    let mut rng = GameRng::seed_from_u64(seed);

    while !spawn_points.is_empty() {
        let spawn_index = rng.gen_range(0..spawn_points.len());
        let spawn_center = spawn_points[spawn_index];

        let mut candidate_accepted = false;
        for _ in 0..rejection_iter {
            let angle: f32 = rng.gen_range(0.0..2.0 * PI);
            let dir = Vec2::new(angle.sin(), angle.cos());
            let radius = rng.gen_range(min_radius..max_radius);

            let candidate =
                (spawn_center + dir * rng.gen_range(radius..2.0 * radius)).extend(radius);

            if !is_valid_variable_radii(candidate, region_size, cell_size, &points, &grid) {
                continue;
            }

            points.push(candidate);
            spawn_points.push(candidate.xy());
            grid[(candidate.y / cell_size).floor() as usize]
                [(candidate.x / cell_size).floor() as usize] = points.len() as i32 - 1;
            candidate_accepted = true;
            break;
        }

        if !candidate_accepted {
            spawn_points.remove(spawn_index);
        }
    }
    points
}

fn is_valid(
    candidate: Vec2,
    region_size: Vec2,
    cell_size: f32,
    radius: f32,
    points: &[Vec2],
    grid: &[Vec<i32>],
) -> bool {
    if candidate.x < 0.0
        || candidate.x >= region_size.x
        || candidate.y < 0.0
        || candidate.y >= region_size.y
    {
        return false;
    }

    let cell_x = (candidate.x / cell_size) as i32;
    let cell_y = (candidate.y / cell_size) as i32;

    // We use -2 as an offset here because we want to search
    // a perimeter of 5x5 around the center cell.
    let search_start_x = (cell_x - 2).max(0) as usize;
    let search_end_x = (cell_x + 2).min(grid[0].len() as i32) as usize;
    let search_start_y = (cell_y - 2).max(0) as usize;
    let search_end_y = (cell_y + 2).min(grid.len() as i32) as usize;

    #[allow(clippy::needless_range_loop)]
    for x in search_start_x..search_end_x {
        #[allow(clippy::needless_range_loop)]
        for y in search_start_y..search_end_y {
            let point_index = grid[y][x];
            if point_index != -1 {
                let distance = (candidate - points[point_index.max(0) as usize]).length_squared();
                if distance < radius.powi(2) {
                    return false;
                }
            }
        }
    }
    true
}

fn is_valid_variable_radii(
    candidate: Vec3,
    region_size: Vec2,
    cell_size: f32,
    points: &[Vec3],
    grid: &[Vec<i32>],
) -> bool {
    if candidate.x < 0.0
        || candidate.x >= region_size.x
        || candidate.y < 0.0
        || candidate.y >= region_size.y
    {
        return false;
    }

    let cell_x = (candidate.x / cell_size) as i32;
    let cell_y = (candidate.y / cell_size) as i32;

    // We use -2 as an offset here because we want to search
    // a perimeter of 5x5 around the center cell.
    let search_start_x = (cell_x - 2).max(0) as usize;
    let search_end_x = (cell_x + 2).min(grid[0].len() as i32) as usize;
    let search_start_y = (cell_y - 2).max(0) as usize;
    let search_end_y = (cell_y + 2).min(grid.len() as i32) as usize;

    #[allow(clippy::needless_range_loop)]
    for x in search_start_x..search_end_x {
        #[allow(clippy::needless_range_loop)]
        for y in search_start_y..search_end_y {
            let point_index = grid[y][x];
            if point_index > -1 {
                let p = points[point_index.max(0) as usize];
                let distance = (candidate.xy() - p.xy()).length_squared();
                if distance < candidate.z.powi(2) + p.z.powi(2) {
                    return false;
                }
            }
        }
    }
    true
}
