use bevy::prelude::*;
use bevy_tweening::{lens::*, *};

use crate::{
    npc::{Npc, NpcDialogue},
    player::Player,
    GameAssets, GameState,
};

use super::camera::YSort;

const FADE_OUT_DURATION: f32 = 2.0;
const START_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
const END_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 1.0);

const BACKGROUND_SPRITE_YSORT: f32 = 40_000.0;
const CHARACTER_YSORT: f32 = 50_000.0;

#[derive(Event)]
pub struct EndingTriggered {
    pub dialogue: NpcDialogue,
}

fn increase_ysorts(
    mut q_player: Query<&mut YSort, With<Player>>,
    mut q_npcs: Query<(&mut YSort, &Npc), Without<Player>>,
    mut ev_ending_triggered: EventReader<EndingTriggered>,
) {
    let mut player_ysort = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    for ev in ev_ending_triggered.read() {
        *player_ysort = YSort(CHARACTER_YSORT);

        for (mut npc_ysort, npc) in &mut q_npcs {
            if npc.dialogue == ev.dialogue {
                *npc_ysort = YSort(CHARACTER_YSORT);
            }
        }
    }

    // TODO
    // 3. Text that says which ending
    // 4. ? Then fade to complete black after some seconds?
    // 5. FIN
}

fn fade_in_black_sprite(mut commands: Commands, assets: Res<GameAssets>) {
    let tween = Tween::new(
        EaseFunction::CubicIn,
        std::time::Duration::from_secs_f32(FADE_OUT_DURATION),
        SpriteColorLens {
            start: START_COLOR,
            end: END_COLOR,
        },
    );

    commands.spawn((
        YSort(BACKGROUND_SPRITE_YSORT),
        Animator::new(tween),
        SpriteBundle {
            texture: assets.white_pixel.clone(),
            transform: Transform::from_scale(Vec3::splat(10_000.0)),
            sprite: Sprite {
                color: START_COLOR,
                ..default()
            },
            ..default()
        },
    ));
}

pub struct EndingPlugin;

impl Plugin for EndingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EndingTriggered>().add_systems(
            Update,
            (
                increase_ysorts,
                fade_in_black_sprite.run_if(on_event::<EndingTriggered>()),
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
