use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;

use crate::{npc::Npc, GameState};

use super::{input::PlayerInput, Player, PlayerState};

const NPC_PROXIMITY_DISTANCE: f32 = 50.0;

#[derive(Event)]
pub struct PlayerStartedChat(pub String);
#[derive(Event)]
pub struct PlayerStoppedChat;

fn start_chat(
    player_input: Res<PlayerInput>,
    q_player: Query<&Transform, With<Player>>,
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

    let player = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    for (npc_transform, npc) in &q_npcs {
        if player
            .translation
            .xy()
            .distance_squared(npc_transform.translation.xy())
            <= NPC_PROXIMITY_DISTANCE.powi(2)
        {
            ev_player_started_chat.send(PlayerStartedChat(npc.dialogue.clone()));
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
