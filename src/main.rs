#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod assets;
mod npc;
mod player;
mod ui;
mod utils;
mod world;

pub use assets::GameAssets;
pub type GameRng = rand_xoshiro::Xoshiro256PlusPlus;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode};

use bevy_asset_loader::prelude::*;
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::Animation2DPlugin;
use bevy_tweening::*;
use bevy_yarnspinner::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.95, 0.90, 0.75);

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    Gaming,
    GameOver,
    Win,
}

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Fifo,
                        mode: WindowMode::Windowed,
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .build(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin {
                enabled: false,
                ..default()
            },
            ParticleSystemPlugin::default(),
            Animation2DPlugin,
            YarnSpinnerPlugin::with_yarn_sources([
                YarnFileSource::file("dialogue/eleonore.yarn"),
                YarnFileSource::file("dialogue/joanna.yarn"),
            ]),
            TweeningPlugin,
        ))
        .insert_resource(Msaa::Off)
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Gaming)
                .load_collection::<GameAssets>(),
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins((
            world::WorldPlugin,
            ui::UiPlugin,
            player::PlayerPlugin,
            npc::NpcPlugin,
            utils::UtilsPlugin,
        ))
        .run();
}
