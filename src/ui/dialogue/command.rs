use std::str::FromStr;

use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;

use crate::{
    npc::{Npc, NpcDialogue},
    player::{chat::PlayerStoppedChat, Player, PlayerState},
    world::ending::EndingTriggered,
};

use super::runner::RunnerFlags;

pub fn stop_chat_command(
    In(_): In<()>,
    q_player: Query<&Player>,
    mut q_runner_flags: Query<&mut RunnerFlags>,
    mut ev_player_stopped_chat: EventWriter<PlayerStoppedChat>,
) {
    let player = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if player.state != PlayerState::Talking {
        error!("Got 'stop_chat_command' but player is not currently talking! Should never happend");
        return;
    }

    for mut flags in &mut q_runner_flags {
        if flags.active {
            // We are setting the line to `...` because you are always
            // supposed to follow a <<stop_chat>> with a `...`.
            // The reason we have to set this explicitely is because
            // the `PresentLineEvent` doesn't fire quickly enough
            // and doesn't update the `flags.line` automatically.
            // This results in a problem with the displayed lines
            // (see `https://github.com/PraxTube/lost-oppai/issues/14`).
            flags.line = Some(LocalizedLine {
                id: LineId(String::new()),
                text: "...".to_string(),
                attributes: Vec::new(),
                metadata: Vec::new(),
                assets: LineAssets::new(),
            });
        }
    }

    ev_player_stopped_chat.send(PlayerStoppedChat);
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
