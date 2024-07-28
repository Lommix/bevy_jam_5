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
        Eatable, ItemAsset, ItemBundle, ItemPlugin, Plantable,
        Quantity, Tags,
    };
}

pub struct ItemPlugin;
impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_tags);
        app.add_plugins(inventory::InventoryPlugin);
        app.init_asset_loader::<ItemAssetLoader>();
        app.init_asset::<ItemAsset>();
    }
}

fn add_tags(
    mut cmd: Commands,
    items: Query<(Entity, &Handle<ItemAsset>), Added<Quantity>>,
    assets: Res<Assets<ItemAsset>>,
) {
    items.iter().for_each(|(entity, handle)| {
        let Some(item) = assets.get(handle) else {
            return;
        };

        let mut cmd = cmd.entity(entity);
        for tag in item.tags.iter() {
            tag.add_comp(&mut cmd);
        }
    });
}

#[derive(Bundle)]
pub struct ItemBundle {
    pub item: Handle<ItemAsset>,
    pub quantity: Quantity,
}

#[derive(Deserialize, Serialize, Asset, TypePath, Debug, Clone)]
pub struct ItemAsset {
    pub name: String,
    pub description: String,
    pub icon: String,
    pub value: f32,
    pub rarity: Rarity,
    pub tags: Vec<Tags>,
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

#[derive(Default, Reflect, Deserialize, Serialize, Debug, Clone)]
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
