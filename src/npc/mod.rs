mod spawn;

use strum_macros::{Display, EnumString};

use bevy::prelude::*;

use crate::player::Player;

#[derive(Clone, Copy, Display, PartialEq, EnumString)]
pub enum NpcDialogue {
    Eleonore,
    Jotem,
    Isabelle,
    Ionas,
    Antonius,
    IonasAndAntonius,
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
    /// The dialogue of the NPC.
    /// This essentially marks which NPC this is.
    pub dialogue: NpcDialogue,
    /// Whether or not this NPC was talked to by the player.
    /// This is used in order to know whether or not to set the flag
    /// on NPCs that want the player to talk to this NPC.
    /// For example, when Jotem wants the player to talk to Eleonore,
    /// we set this flag to true on Eleonore when the player talks to her
    /// and then Jotem knows that the player talked to Eleonore.
    pub was_talked_to: bool,
    /// All the NPCs that mentioned this NPC in a conversation.
    /// For example, when Jotem talks with the player about Eleonore,
    /// then we add Jotem to this Vec on Eleonore.
    /// This way, we can have different dialogue options based on whether the
    /// player already knows about the NPCs or not.
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
    mut q_npcs: Query<(&Transform, &mut Sprite), (With<Npc>, Without<Player>)>,
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
