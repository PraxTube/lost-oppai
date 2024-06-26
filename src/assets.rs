use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_trickfilm::prelude::*;

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

    #[asset(path = "map/water_sparkles.png")]
    pub water_sparkles: Handle<Image>,

    // --- FLORA ---
    #[asset(path = "map/flora/tree.png")]
    pub tree: Handle<Image>,
    #[asset(path = "map/flora/tree_trunk.png")]
    pub tree_trunk: Handle<Image>,
    #[asset(path = "map/flora/tree_shadow.png")]
    pub tree_shadow: Handle<Image>,
    #[asset(path = "map/flora/tree_pedal.png")]
    pub tree_pedal: Handle<Image>,

    #[asset(path = "map/flora/bush1.png")]
    pub bush1: Handle<Image>,
    #[asset(path = "map/flora/bush2.png")]
    pub bush2: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 3, rows = 1))]
    #[asset(path = "map/rocks.png")]
    pub rocks: Handle<TextureAtlas>,

    // --- FAUNA ---
    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 8, rows = 2))]
    #[asset(path = "map/fauna/bird.png")]
    pub bird: Handle<TextureAtlas>,
    #[asset(
        paths(
            "map/fauna/bird.trickfilm#idle",
            "map/fauna/bird.trickfilm#jump",
            "map/fauna/bird.trickfilm#pick",
            "map/fauna/bird.trickfilm#fly",
        ),
        collection(typed)
    )]
    pub bird_animations: Vec<Handle<AnimationClip2D>>,
    #[asset(path = "map/fauna/bird_shadow.png")]
    pub bird_shadow: Handle<Image>,

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

    #[asset(texture_atlas(tile_size_x = 34.0, tile_size_y = 34.0, columns = 3, rows = 1))]
    #[asset(path = "ui/keys/down_key.png")]
    pub ui_down_key: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 34.0, tile_size_y = 34.0, columns = 3, rows = 1))]
    #[asset(path = "ui/keys/up_key.png")]
    pub ui_up_key: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 34.0, tile_size_y = 34.0, columns = 3, rows = 1))]
    #[asset(path = "ui/keys/left_key.png")]
    pub ui_left_key: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 34.0, tile_size_y = 34.0, columns = 3, rows = 1))]
    #[asset(path = "ui/keys/right_key.png")]
    pub ui_right_key: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 66.0, tile_size_y = 34.0, columns = 2, rows = 1))]
    #[asset(path = "ui/keys/shift_key.png")]
    pub ui_shift_key: Handle<TextureAtlas>,
    #[asset(
        paths(
            "ui/keys/keys.trickfilm#key",
            "ui/keys/keys.trickfilm#arrow",
            "ui/keys/keys.trickfilm#shift"
        ),
        collection(typed)
    )]
    pub ui_keys_animations: Vec<Handle<AnimationClip2D>>,

    // --- AUDIO ---
    #[asset(path = "audio/player_footstep.ogg")]
    pub player_footstep: Handle<AudioSource>,

    #[asset(path = "audio/bird_step.ogg")]
    pub bird_step_sound: Handle<AudioSource>,
    #[asset(path = "audio/birds.ogg")]
    pub bird_sounds: Handle<AudioSource>,
    #[asset(path = "audio/crickets.ogg")]
    pub cricket_sounds: Handle<AudioSource>,

    #[asset(path = "audio/eleonore_flap_sound.ogg")]
    pub eleonore_flap_sound: Handle<AudioSource>,

    #[asset(path = "audio/pai_blip.ogg")]
    pub pai_blip_sound: Handle<AudioSource>,
    #[asset(path = "audio/eleonore_blip.ogg")]
    pub eleonore_blip_sound: Handle<AudioSource>,
    #[asset(path = "audio/joanna_blip.ogg")]
    pub joanna_blip_sound: Handle<AudioSource>,

    // --- FONT ---
    #[asset(path = "fonts/PressStart2P.ttf")]
    pub font: Handle<Font>,
}
