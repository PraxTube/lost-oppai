use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{audio::PlaySound, GameAssets, GameState};

use super::Eleonore;

fn play_elenore_flap_sound(
    assets: Res<GameAssets>,
    mut q_eleonore: Query<(Entity, &AnimationPlayer2D, &mut Eleonore)>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    let (entity, player, mut eleonore) = match q_eleonore.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if !player.is_finished() {
        eleonore.is_playing_flap_sound = false;
    }

    if player.is_finished() && !eleonore.is_playing_flap_sound {
        eleonore.is_playing_flap_sound = true;
        ev_play_sound.send(PlaySound {
            clip: assets.eleonore_flap_sound.clone(),
            volume: 1.5,
            parent: Some(entity),
            ..default()
        });
    }
}

pub struct NpcAudioPlugin;

impl Plugin for NpcAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (play_elenore_flap_sound,).run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
