use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;

use crate::{npc::Npc, GameState};

use super::{input::PlayerInput, Player, PlayerState, NPC_PROXIMITY_DISTANCE};

#[derive(Event)]
pub struct PlayerStartedChat {
    pub dialogue: String,
    pub direction: Vec2,
}
#[derive(Event)]
pub struct PlayerStoppedChat;

fn start_chat(
    player_input: Res<PlayerInput>,
    mut q_player: Query<(&Transform, &mut Player)>,
    q_npcs: Query<(&Transform, &Npc), Without<Player>>,
    q_dialogue_runners: Query<With<DialogueRunner>>,
    mut ev_player_started_chat: EventWriter<PlayerStartedChat>,
) {
    if !player_input.start_dialogue {
        return;
    }
    if !q_dialogue_runners.is_empty() {
        return;
    }

    let (player_transform, mut player) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    for (npc_transform, npc) in &q_npcs {
        if player_transform
            .translation
            .xy()
            .distance_squared(npc_transform.translation.xy())
            <= NPC_PROXIMITY_DISTANCE.powi(2)
        {
            player.state = PlayerState::Talking;
            ev_player_started_chat.send(PlayerStartedChat {
                dialogue: npc.dialogue.clone(),
                direction: npc_transform.translation.xy() - player_transform.translation.xy(),
            });
            break;
        }
    }
}

fn stop_chat(
    player_input: Res<PlayerInput>,
    q_player: Query<&Player>,
    mut ev_player_stopped_chat: EventWriter<PlayerStoppedChat>,
) {
    if !player_input.escape {
        return;
    }

    let player = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    if player.state != PlayerState::Talking {
        return;
    }

    ev_player_stopped_chat.send(PlayerStoppedChat);
}

pub struct PlayerChatPlugin;

impl Plugin for PlayerChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerStartedChat>()
            .add_event::<PlayerStoppedChat>()
            .add_systems(
                Update,
                (start_chat, stop_chat).run_if(in_state(GameState::Gaming)),
            );
    }
}
