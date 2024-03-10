use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_trickfilm::prelude::*;
// use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 128.0, columns = 10, rows = 12))]
    #[asset(path = "player/player.png")]
    pub player: Handle<TextureAtlas>,
    #[asset(
        paths(
            "player/player.trickfilm#idle-down",
            "player/player.trickfilm#idle-left",
            "player/player.trickfilm#idle-right",
            "player/player.trickfilm#idle-top",
            "player/player.trickfilm#walking-down",
            "player/player.trickfilm#walking-left",
            "player/player.trickfilm#walking-right",
            "player/player.trickfilm#walking-top",
            "player/player.trickfilm#running-down",
            "player/player.trickfilm#running-left",
            "player/player.trickfilm#running-right",
            "player/player.trickfilm#running-top",
        ),
        collection(typed)
    )]
    pub player_animations: Vec<Handle<AnimationClip2D>>,

    // --- NPC ---
    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 9, rows = 1))]
    #[asset(path = "npc/npc.png")]
    pub npc: Handle<TextureAtlas>,
    #[asset(paths("npc/npc.trickfilm#idle",), collection(typed))]
    pub npc_animations: Vec<Handle<AnimationClip2D>>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 9, rows = 1))]
    #[asset(path = "npc/shadow.png")]
    pub npc_shadow: Handle<TextureAtlas>,

    // --- MAP ---
    #[asset(path = "map/tileset.png")]
    pub tileset: Handle<Image>,

    // --- UI ---
    #[asset(path = "ui/white_pixel.png")]
    pub white_pixel: Handle<Image>,

    #[asset(path = "ui/dialogue_edge.png")]
    pub dialogue_edge: Handle<Image>,
    #[asset(path = "ui/dialogue_continue.png")]
    pub dialogue_continue: Handle<Image>,

    // --- FONT ---
    #[asset(path = "fonts/PressStart2P.ttf")]
    pub font: Handle<Font>,
}
