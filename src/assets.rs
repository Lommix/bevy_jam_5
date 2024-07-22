use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/rise_of_spirit.ogg")]
    pub menu_music: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct ConfigAssets {}

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {}
