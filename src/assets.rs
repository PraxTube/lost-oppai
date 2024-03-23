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
    #[asset(path = "npc/eleonore.png")]
    pub eleonore: Handle<TextureAtlas>,
    #[asset(paths("npc/eleonore.trickfilm#idle",), collection(typed))]
    pub eleonore_animations: Vec<Handle<AnimationClip2D>>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 9, rows = 1))]
    #[asset(path = "npc/eleonore_shadow.png")]
    pub eleonore_shadow: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 18, rows = 1))]
    #[asset(path = "npc/joanna.png")]
    pub joanna: Handle<TextureAtlas>,
    #[asset(paths("npc/joanna.trickfilm#idle",), collection(typed))]
    pub joanna_animatins: Vec<Handle<AnimationClip2D>>,
    #[asset(path = "npc/joanna_shadow.png")]
    pub joanna_shadow: Handle<Image>,

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
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 2, rows = 1))]
    #[asset(path = "ui/dialogue_start_hint.png")]
    pub dialogue_start_hint: Handle<TextureAtlas>,
    #[asset(paths("ui/dialogue_start_hint.trickfilm#main",), collection(typed))]
    pub dialogue_start_hint_animations: Vec<Handle<AnimationClip2D>>,

    // --- FONT ---
    #[asset(path = "fonts/PressStart2P.ttf")]
    pub font: Handle<Font>,
}
