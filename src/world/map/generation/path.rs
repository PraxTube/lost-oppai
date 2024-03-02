use rand::{Rng, SeedableRng};

use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d_seeded;

use crate::{GameRng, GameState};

use super::{BitMap, GRASS_TYPE_INDEX, PATH_TYPE_INDEX};

const NOISE_ZOOM: f32 = 0.02;

fn generate_bezier_curve_points(distance: f32, sample_size: usize, seed: u64) -> Vec<IVec2> {
    let mut rng = GameRng::seed_from_u64(seed);
    let x = rng.gen_range(-distance..distance);
    let y_sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
    let y = y_sign * (distance.powi(2) - x.powi(2)).sqrt();
    let p1 = Vec2::ZERO;
    let p2 = Vec2::new(x, y);

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

fn generate_path(mut bitmap: ResMut<BitMap>) {
    let distance = 200.0;
    let sample_size = 200;
    let min_radius: i32 = 1;
    let max_radius: i32 = min_radius + 1;
    let min_radius_grass: i32 = max_radius + 1;
    let max_radius_grass: i32 = min_radius_grass + 3;

    let points = generate_bezier_curve_points(distance, sample_size, bitmap.seed() as u64);

    for v in points {
        let w = Vec2::new(v.x as f32, v.y as f32);

        let noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, bitmap.seed());
        let secondary_noise = simplex_noise_2d_seeded(w * NOISE_ZOOM, bitmap.seed() + 1.0) * 1.0;
        let radius = (min_radius as f32
            + 0.25 * (noise + secondary_noise + 2.0) * (max_radius - min_radius) as f32)
            as i32;
        let sqrt_radius = radius.pow(2);
        let radius_grass = (min_radius_grass as f32
            + 0.25 * (noise + secondary_noise + 2.0) * (max_radius_grass - min_radius_grass) as f32)
            as i32;
        let sqrt_radius_grass = radius_grass.pow(2);

        for x in -radius_grass..=radius_grass {
            for y in -radius_grass..=radius_grass {
                let offset = IVec2::new(x, y);
                let dis = offset.length_squared();
                if dis < sqrt_radius {
                    bitmap.set_type_index(v + offset, PATH_TYPE_INDEX);
                } else if dis < sqrt_radius_grass
                    && bitmap.get_tileset_raw(v + offset).0 != PATH_TYPE_INDEX
                {
                    bitmap.set_type_index(v + offset, GRASS_TYPE_INDEX);
                }
            }
        }
    }
}

pub struct PathGenerationPlugin;

impl Plugin for PathGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), (generate_path,));
    }
}
