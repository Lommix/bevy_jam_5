use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/rise_of_spirit.ogg")]
    pub menu_music: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(path = "raw/icons.aseprite")]
    pub icons: Handle<Aseprite>,
    #[asset(path = "raw/tile.aseprite")]
    pub tile: Handle<Aseprite>,
    #[asset(path = "raw/house.aseprite")]
    pub house: Handle<Aseprite>,
    #[asset(path = "raw/field.aseprite")]
    pub field: Handle<Aseprite>,
}
