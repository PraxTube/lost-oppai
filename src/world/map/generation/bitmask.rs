use std::collections::HashMap;

use rand::{thread_rng, Rng};

use super::{
    BITMASK_BOT_LEFT, BITMASK_BOT_RIGHT, BITMASK_TOP_LEFT, BITMASK_TOP_RIGHT, INVALID_TILE,
};

pub const GRASS_FLOWER_SUPER_POSITION: u16 = 15;

fn grid_to_index(x: u16, y: u16) -> u16 {
    x + y * 16
}

pub struct BitMasks {
    masks: HashMap<u16, Vec<u16>>,
}

impl BitMasks {
    pub fn grass() -> Self {
        Self {
            masks: HashMap::from([
                (
                    0,
                    vec![
                        grid_to_index(11, 13),
                        grid_to_index(12, 13),
                        grid_to_index(13, 13),
                        grid_to_index(12, 14),
                        grid_to_index(13, 14),
                    ],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(0, 1)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT,
                    vec![grid_to_index(5, 0)],
                ),
                (
                    BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(8, 0)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(8, 2)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_LEFT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(5, 2)],
                ),
                (
                    BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT,
                    vec![grid_to_index(6, 0), grid_to_index(7, 0)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(6, 2), grid_to_index(7, 2)],
                ),
                (
                    BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(7, 1), grid_to_index(8, 1)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_LEFT,
                    vec![grid_to_index(5, 1), grid_to_index(6, 1)],
                ),
                (BITMASK_BOT_RIGHT, vec![grid_to_index(5, 3)]),
                (BITMASK_BOT_LEFT, vec![grid_to_index(6, 3)]),
                (BITMASK_TOP_LEFT, vec![grid_to_index(6, 4)]),
                (BITMASK_TOP_RIGHT, vec![grid_to_index(5, 4)]),
                (
                    BITMASK_TOP_LEFT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(7, 3)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_RIGHT,
                    vec![grid_to_index(7, 4)],
                ),
            ]),
        }
    }

    pub fn path() -> Self {
        Self {
            masks: HashMap::from([
                (0, vec![GRASS_FLOWER_SUPER_POSITION]),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(2, 1)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT,
                    vec![grid_to_index(1, 3)],
                ),
                (
                    BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(2, 3)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(2, 4)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_LEFT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(1, 4)],
                ),
                (
                    BITMASK_TOP_LEFT | BITMASK_TOP_RIGHT,
                    vec![grid_to_index(2, 2), grid_to_index(4, 5)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(2, 0), grid_to_index(3, 5)],
                ),
                (
                    BITMASK_TOP_RIGHT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(1, 1), grid_to_index(2, 5)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_LEFT,
                    vec![grid_to_index(3, 1), grid_to_index(1, 5)],
                ),
                (BITMASK_BOT_RIGHT, vec![grid_to_index(1, 0)]),
                (BITMASK_BOT_LEFT, vec![grid_to_index(3, 0)]),
                (BITMASK_TOP_LEFT, vec![grid_to_index(3, 2)]),
                (BITMASK_TOP_RIGHT, vec![grid_to_index(1, 2)]),
                (
                    BITMASK_TOP_LEFT | BITMASK_BOT_RIGHT,
                    vec![grid_to_index(3, 4)],
                ),
                (
                    BITMASK_BOT_LEFT | BITMASK_TOP_RIGHT,
                    vec![grid_to_index(3, 3)],
                ),
            ]),
        }
    }

    pub fn flower() -> Self {
        Self {
            masks: HashMap::from([
                (
                    0,
                    vec![
                        grid_to_index(0, 1),
                        grid_to_index(0, 2),
                        grid_to_index(0, 3),
                        grid_to_index(0, 4),
                    ],
                ),
                (
                    1,
                    vec![
                        grid_to_index(0, 6),
                        grid_to_index(1, 6),
                        grid_to_index(2, 6),
                        grid_to_index(3, 6),
                        grid_to_index(0, 7),
                        grid_to_index(1, 7),
                        grid_to_index(2, 7),
                        grid_to_index(3, 7),
                    ],
                ),
            ]),
        }
    }
}

impl BitMasks {
    pub fn get_index(&self, mask: u16) -> u16 {
        let binding = vec![INVALID_TILE];
        let indices = self.masks.get(&mask).unwrap_or(&binding);
        let mut rng = thread_rng();
        let index = rng.gen_range(0..indices.len());
        indices[index]
    }
}
