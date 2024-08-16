#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod assets;
mod audio;
mod npc;
mod player;
mod ui;
mod utils;
mod world;

pub use assets::GameAssets;
pub type GameRng = rand_xoshiro::Xoshiro256PlusPlus;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode, WindowResolution};

use bevy_asset_loader::prelude::*;
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::Animation2DPlugin;
use bevy_tweening::*;
use bevy_yarnspinner::prelude::*;

const BACKGROUND_COLOR: Color = Color::BLACK;
const DEFAULT_WINDOW_WIDTH: f32 = 1280.0;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    MainMenu,
    Gaming,
    Ending,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Fifo,
                        mode: WindowMode::Windowed,
                        resizable: false,
                        fit_canvas_to_parent: false,
                        canvas: Some("#game-canvas".to_string()),
                        resolution: WindowResolution::new(
                            DEFAULT_WINDOW_WIDTH,
                            DEFAULT_WINDOW_WIDTH * 9.0 / 16.0,
                        ),
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin {
                enabled: false,
                ..default()
            },
            ParticleSystemPlugin,
            Animation2DPlugin,
            YarnSpinnerPlugin::with_yarn_sources([
                YarnFileSource::file("dialogue/eleonore.yarn"),
                YarnFileSource::file("dialogue/jotem.yarn"),
                YarnFileSource::file("dialogue/isabelle.yarn"),
                YarnFileSource::file("dialogue/ionas-and-antonius.yarn"),
                YarnFileSource::file("dialogue/paladins.yarn"),
            ])
            .with_development_file_generation(DevelopmentFileGeneration::None),
            TweeningPlugin,
        ))
        .insert_resource(Msaa::Off)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::MainMenu)
                .load_collection::<GameAssets>(),
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins((
            world::WorldPlugin,
            ui::UiPlugin,
            audio::GameAudioPlugin,
            player::PlayerPlugin,
            npc::NpcPlugin,
            utils::UtilsPlugin,
        ))
        .run();
}
