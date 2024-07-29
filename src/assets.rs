use crate::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/rise_of_spirit.ogg")]
    pub menu_music: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "big_blue.TTF")]
    pub font: Handle<Font>,
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
    #[asset(path = "raw/workshop.aseprite")]
    pub workshop: Handle<Aseprite>,
    #[asset(path = "raw/construct.aseprite")]
    pub construct: Handle<Aseprite>,
    #[asset(path = "raw/market.aseprite")]
    pub market: Handle<Aseprite>,
}

#[derive(AssetCollection, Resource)]
pub struct ItemAssets {
    #[asset(path = "items/vodka.item.ron")]
    pub vodka: Handle<ItemAsset>,
    #[asset(path = "items/beer.item.ron")]
    pub beer: Handle<ItemAsset>,
    #[asset(path = "items/rum.item.ron")]
    pub rum: Handle<ItemAsset>,

    #[asset(path = "items/wood.item.ron")]
    pub wood: Handle<ItemAsset>,
    #[asset(path = "items/corn.item.ron")]
    pub corn: Handle<ItemAsset>,
    #[asset(path = "items/carrot.item.ron")]
    pub carrot: Handle<ItemAsset>,
    #[asset(path = "items/wheat.item.ron")]
    pub wheat: Handle<ItemAsset>,
    #[asset(path = "items/grape.item.ron")]
    pub grape: Handle<ItemAsset>,
    #[asset(path = "items/potato.item.ron")]
    pub potato: Handle<ItemAsset>,
    #[asset(path = "items/sugarcane.item.ron")]
    pub sugarcane: Handle<ItemAsset>,
}

#[derive(AssetCollection, Resource)]
pub struct WorkOrderAssets {
    #[asset(path = "orders/vodka.work.ron")]
    pub distill_vodka: Handle<WorkOrder>,
    #[asset(path = "orders/potatos.work.ron")]
    pub grow_potatos: Handle<WorkOrder>,
    #[asset(path = "orders/carrots.work.ron")]
    pub grow_carrots: Handle<WorkOrder>,
    #[asset(path = "orders/wood.work.ron")]
    pub wood: Handle<WorkOrder>,
}

#[derive(AssetCollection, Resource)]
pub struct BuildingAssets {
    #[asset(path = "buildings/house.build.ron")]
    pub house: Handle<BuildingAsset>,
    #[asset(path = "buildings/field.build.ron")]
    pub field: Handle<BuildingAsset>,
    #[asset(path = "buildings/workshop.build.ron")]
    pub workshop: Handle<BuildingAsset>,
    #[asset(path = "buildings/market.build.ron")]
    pub market: Handle<BuildingAsset>,
    #[asset(path = "buildings/lumbermill.build.ron")]
    pub lumbermill: Handle<BuildingAsset>,
    #[asset(path = "buildings/statue.build.ron")]
    pub statue: Handle<BuildingAsset>,
}

impl BuildingAssets {
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = Handle<BuildingAsset>> {
        vec![
            self.house.clone(),
            self.field.clone(),
            self.workshop.clone(),
            self.market.clone(),
            self.lumbermill.clone(),
            self.statue.clone(),
        ]
        .into_iter()
    }
}
