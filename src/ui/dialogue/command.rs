use std::str::FromStr;

use bevy::prelude::*;

use crate::{
    npc::{Npc, NpcDialogue},
    player::{chat::PlayerStoppedChat, Player, PlayerState},
    world::ending::EndingTriggered,
    GameState,
};

const DELAY_FRAMES_PLAYER_STOPPED_CHAT: usize = 3;

#[derive(Event)]
pub struct DelayedPlayerStoppedChat;

pub fn stop_chat_command(
    In(_): In<()>,
    q_player: Query<&Player>,
    mut ev_delayed_player_stopped_chat: EventWriter<DelayedPlayerStoppedChat>,
) {
    let player = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if player.state != PlayerState::Talking {
        return;
    }

    ev_delayed_player_stopped_chat.send(DelayedPlayerStoppedChat);
}

pub fn target_npc_mentioned_command(
    In((source_npc, target_npc)): In<(&str, &str)>,
    mut q_npcs: Query<&mut Npc>,
) {
    let source_npc = match NpcDialogue::from_str(source_npc.trim_start_matches('_')) {
        Ok(r) => r,
        Err(err) => {
            error!("Not a valid npc name! {}", err);
            return;
        }
    };

    let target_npc = match NpcDialogue::from_str(target_npc.trim_start_matches('_')) {
        Ok(r) => r,
        Err(err) => {
            error!("Not a valid npc name! {}", err);
            return;
        }
    };

    for mut npc in &mut q_npcs {
        if npc.dialogue == target_npc {
            npc.was_mentioned_by.push(source_npc);
        }
    }
}

pub fn trigger_ending_command(
    In(npc_name): In<&str>,
    mut ev_ending_triggered: EventWriter<EndingTriggered>,
) {
    let dialogue = match NpcDialogue::from_str(npc_name.trim_start_matches('_')) {
        Ok(r) => r,
        Err(err) => {
            error!("Not a valid npc name! {}", err);
            return;
        }
    };
    ev_ending_triggered.send(EndingTriggered { dialogue });
}

fn relay_player_stopped_chat_event(
    mut ev_delayed_player_stopped_chat: EventReader<DelayedPlayerStoppedChat>,
    mut ev_player_stopped_chat: EventWriter<PlayerStoppedChat>,
    mut started: Local<bool>,
    mut frames: Local<usize>,
) {
    for _ev in ev_delayed_player_stopped_chat.read() {
        *started = true;
    }

    if *started {
        if *frames >= DELAY_FRAMES_PLAYER_STOPPED_CHAT {
            *started = false;
            ev_player_stopped_chat.send(PlayerStoppedChat);
        }
        *frames += 1;
    }
}

pub struct DialogueCommandPlugin;

impl Plugin for DialogueCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DelayedPlayerStoppedChat>().add_systems(
            Update,
            (relay_player_stopped_chat_event,).run_if(in_state(GameState::Gaming)),
        );
    }
}
