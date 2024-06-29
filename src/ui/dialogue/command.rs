use bevy::prelude::*;

use crate::{
    player::{chat::PlayerStoppedChat, Player, PlayerState},
    GameState,
};

use super::typewriter::Typewriter;

const DELAY_FRAMES_PLAYER_STOPPED_CHAT: usize = 3;

#[derive(Event)]
pub struct DelayedPlayerStoppedChat;

pub fn set_type_speed_command(In(speed): In<f32>, mut typewriter: ResMut<Typewriter>) {
    typewriter.set_type_speed(speed);
}

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
