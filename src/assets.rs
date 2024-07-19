use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_trickfilm::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "player/player.png")]
    pub player_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 64, tile_size_y = 128, columns = 10, rows = 12))]
    pub player_layout: Handle<TextureAtlasLayout>,
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
    #[asset(path = "npc/eleonore.png")]
    pub eleonore_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 64, tile_size_y = 64, columns = 9, rows = 1))]
    pub eleonore_layout: Handle<TextureAtlasLayout>,
    #[asset(paths("npc/eleonore.trickfilm#idle",), collection(typed))]
    pub eleonore_animations: Vec<Handle<AnimationClip2D>>,
    #[asset(path = "npc/eleonore_shadow.png")]
    pub eleonore_shadow_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 32, tile_size_y = 32, columns = 9, rows = 1))]
    pub eleonore_shadow_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "npc/jotem.png")]
    pub jotem_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 64, tile_size_y = 64, columns = 6, rows = 1))]
    pub jotem_layout: Handle<TextureAtlasLayout>,
    #[asset(paths("npc/jotem.trickfilm#idle",), collection(typed))]
    pub jotem_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(path = "npc/isabelle.png")]
    pub isabelle_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 48, tile_size_y = 64, columns = 7, rows = 1))]
    pub isabelle_layout: Handle<TextureAtlasLayout>,
    #[asset(paths("npc/isabelle.trickfilm#idle",), collection(typed))]
    pub isabelle_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(path = "npc/antonius.png")]
    pub antonius_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 80, tile_size_y = 80, columns = 5, rows = 1))]
    pub antonius_layout: Handle<TextureAtlasLayout>,
    #[asset(paths("npc/antonius.trickfilm#idle",), collection(typed))]
    pub antonius_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(path = "npc/ionas.png")]
    pub ionas_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 80, tile_size_y = 80, columns = 5, rows = 1))]
    pub ionas_layout: Handle<TextureAtlasLayout>,
    #[asset(paths("npc/ionas.trickfilm#idle",), collection(typed))]
    pub ionas_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(path = "npc/sven.png")]
    pub sven_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 64, tile_size_y = 80, columns = 8, rows = 1))]
    pub sven_layout: Handle<TextureAtlasLayout>,
    #[asset(paths("npc/sven.trickfilm#idle",), collection(typed))]
    pub sven_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(path = "npc/joanna.png")]
    pub joanna_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 64, tile_size_y = 64, columns = 18, rows = 1))]
    pub joanna_layout: Handle<TextureAtlasLayout>,
    #[asset(paths("npc/joanna.trickfilm#idle",), collection(typed))]
    pub joanna_animatins: Vec<Handle<AnimationClip2D>>,

    #[asset(path = "npc/dorothea.png")]
    pub dorothea_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 32, tile_size_y = 48, columns = 6, rows = 1))]
    pub dorothea_layout: Handle<TextureAtlasLayout>,
    #[asset(paths("npc/dorothea.trickfilm#idle",), collection(typed))]
    pub dorothea_animations: Vec<Handle<AnimationClip2D>>,

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

    #[asset(path = "map/rocks.png")]
    pub rocks_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 16, tile_size_y = 16, columns = 3, rows = 1))]
    pub rocks_layout: Handle<TextureAtlasLayout>,

    // --- FAUNA ---
    #[asset(path = "map/fauna/bird.png")]
    pub bird_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 16, tile_size_y = 16, columns = 8, rows = 2))]
    pub bird_layout: Handle<TextureAtlasLayout>,
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
    #[asset(path = "ui/dialogue_start_hint.png")]
    pub dialogue_start_hint_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 32, tile_size_y = 32, columns = 2, rows = 1))]
    pub dialogue_start_hint_layout: Handle<TextureAtlasLayout>,
    #[asset(paths("ui/dialogue_start_hint.trickfilm#main",), collection(typed))]
    pub dialogue_start_hint_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(path = "ui/keys/down_key.png")]
    pub ui_down_key_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 34, tile_size_y = 34, columns = 3, rows = 1))]
    pub ui_down_key_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "ui/keys/up_key.png")]
    pub ui_up_key_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 34, tile_size_y = 34, columns = 3, rows = 1))]
    pub ui_up_key_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "ui/keys/left_key.png")]
    pub ui_left_key_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 34, tile_size_y = 34, columns = 3, rows = 1))]
    pub ui_left_key_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "ui/keys/right_key.png")]
    pub ui_right_key_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 34, tile_size_y = 34, columns = 3, rows = 1))]
    pub ui_right_key_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "ui/keys/shift_key.png")]
    pub ui_shift_key_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 66, tile_size_y = 34, columns = 2, rows = 1))]
    pub ui_shift_key_layout: Handle<TextureAtlasLayout>,
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
    #[asset(path = "audio/bird_flaps.ogg")]
    pub bird_flaps_sound: Handle<AudioSource>,
    #[asset(path = "audio/birds.ogg")]
    pub bird_sounds: Handle<AudioSource>,
    #[asset(path = "audio/crickets.ogg")]
    pub cricket_sounds: Handle<AudioSource>,

    #[asset(path = "audio/pai_blip.ogg")]
    pub pai_blip_sound: Handle<AudioSource>,
    #[asset(path = "audio/eleonore_blip.ogg")]
    pub eleonore_blip_sound: Handle<AudioSource>,
    #[asset(path = "audio/joanna_blip.ogg")]
    pub joanna_blip_sound: Handle<AudioSource>,
    #[asset(path = "audio/jotem_blip.ogg")]
    pub jotem_blip_sound: Handle<AudioSource>,

    // --- FONT ---
    #[asset(path = "fonts/PressStart2P.ttf")]
    pub pixel_font: Handle<Font>,
    #[asset(path = "fonts/Silver.ttf")]
    pub silver_font: Handle<Font>,
}
