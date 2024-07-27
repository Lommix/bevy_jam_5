use std::future::Future;

use crate::prelude::*;
use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    ecs::system::EntityCommands,
    utils::ConditionalSendFuture,
};
use serde::{Deserialize, Serialize};

mod inventory;

#[allow(unused)]
pub mod prelude {
    pub use super::inventory::Inventory;
    pub use super::{
        Eatable, GoldValue, Item, ItemAsset, ItemBundle, ItemPlugin,
        Plantable, Quantity, SpawnItem, Tags,
    };
}

pub struct ItemPlugin;
impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(inventory::InventoryPlugin);
        app.init_asset_loader::<ItemAssetLoader>();
        app.init_asset::<ItemAsset>();

        app.register_type::<GoldValue>()
            .register_type::<Rarity>()
            .register_type::<Quantity>()
            .register_type::<Item>();
    }
}

#[derive(Bundle)]
pub struct ItemBundle {
    pub item: Item,
    pub value: GoldValue,
    pub rarity: Rarity,
    pub quantity: Quantity,
    pub name: Name,
}

impl Default for ItemBundle {
    fn default() -> Self {
        Self {
            item: Item::default(),
            value: GoldValue::default(),
            rarity: Rarity::default(),
            quantity: Quantity::default(),
            name: Name::new("item"),
        }
    }
}

#[derive(Deserialize, Serialize, Asset, TypePath, Debug, Clone)]
pub struct ItemAsset {
    pub item: Item,
    pub value: GoldValue,
    pub rarity: Rarity,
    pub tags: Vec<Tags>,
}

#[derive(
    Component, Default, Reflect, Debug, Clone, Deserialize, Serialize,
)]
#[reflect]
pub struct Item {
    pub name: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Tags {
    Eatable,
    Sellable,
    Plantable,
}
impl Tags {
    pub fn add_comp(&self, cmd: &mut EntityCommands) {
        match &self {
            Tags::Eatable => cmd.insert(Eatable),
            Tags::Sellable => cmd.insert(Sellable),
            Tags::Plantable => cmd.insert(Plantable),
        };
    }
}

#[derive(
    Default,
    Component,
    Reflect,
    Deref,
    DerefMut,
    Deserialize,
    Serialize,
    Debug,
    Clone,
)]
#[reflect]
pub struct GoldValue(pub f32);

#[derive(
    Default,
    Reflect,
    Component,
    Clone,
    Deref,
    DerefMut,
    Deserialize,
    Serialize,
    Debug,
)]
#[reflect]
pub struct Quantity(pub i32);

#[derive(
    Default, Reflect, Component, Deserialize, Serialize, Debug, Clone,
)]
#[reflect]
pub enum Rarity {
    #[default]
    Common,
    Rare,
    Epic,
    Legendary,
}

#[derive(Component, Deserialize, Serialize)]
pub struct Eatable;

#[derive(Component, Deserialize, Serialize)]
pub struct Plantable;

#[derive(Component, Deserialize, Serialize)]
pub struct Sellable;

pub trait SpawnItem {
    fn spawn_item(
        &mut self,
        asset: &ItemAsset,
        quantity: i32,
    ) -> EntityCommands;
}
impl SpawnItem for Commands<'_, '_> {
    fn spawn_item(
        &mut self,
        asset: &ItemAsset,
        quantity: i32,
    ) -> EntityCommands {
        let mut item = self.spawn((
            asset.item.clone(),
            asset.rarity.clone(),
            asset.value.clone(),
            Quantity(quantity),
        ));

        for tag in asset.tags.iter() {
            tag.add_comp(&mut item);
        }

        item
    }
}

#[derive(Default)]
pub struct ItemAssetLoader;
impl AssetLoader for ItemAssetLoader {
    type Asset = ItemAsset;
    type Settings = ();
    type Error = anyhow::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> impl ConditionalSendFuture
           + Future<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let item: ItemAsset =
                ron::de::from_bytes(bytes.as_slice())?;
            Ok(item)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["item.ron"]
    }
}
