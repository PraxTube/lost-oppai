mod spawn;

use strum_macros::{Display, EnumString};

use bevy::prelude::*;

use crate::player::Player;

#[derive(Clone, Copy, Display, PartialEq, EnumString)]
pub enum NpcDialogue {
    Eleonore,
    Joanna,
    Jotem,
    Isabelle,
    IonasAndAntonius,
    Ionas,
    Antonius,
}

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(spawn::NpcSpawnPlugin)
            .add_systems(Update, (face_player,));
    }
}

#[derive(Component)]
pub struct Npc {
    pub dialogue: NpcDialogue,
    pub was_talked_to: bool,
    pub was_mentioned_by: Vec<NpcDialogue>,
}

impl Npc {
    fn new(dialogue: NpcDialogue) -> Self {
        Self {
            dialogue,
            was_talked_to: false,
            was_mentioned_by: Vec::new(),
        }
    }
}

fn face_player(
    q_player: Query<&Transform, With<Player>>,
    mut q_npcs: Query<(&Transform, &mut TextureAtlasSprite), (With<Npc>, Without<Player>)>,
) {
    let player = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    for (transform, mut sprite) in &mut q_npcs {
        let flip = player.translation.x < transform.translation.x;
        sprite.flip_x = flip;
    }
}
