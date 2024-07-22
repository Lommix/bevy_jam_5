pub use crate::prelude::*;
use bevy::ecs::{component::ComponentId, world::DeferredWorld};

pub mod prelude {
    pub use super::{GoldValue, Inventory, InventoryPlugin, Item};
}

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.world_mut()
            .register_component_hooks::<Inventory>()
            .on_remove(inventory_removed);
    }
}

fn inventory_removed(
    world: DeferredWorld,
    entity: Entity,
    id: ComponentId,
) {
    info!("inventory removed, deleting items");
    let inventory = world.get::<Inventory>(entity);
    dbg!(&inventory);
}

#[derive(Component, Debug, DerefMut, Deref)]
pub struct Inventory(pub Vec<Entity>);

#[derive(Bundle)]
pub struct ItemBundle {
    item: Item,
    value: GoldValue,
    rarity: Rarity,
    quantity: Quantity,
}

#[derive(Component)]
pub struct Item {
    pub name: String,
    pub description: String,
}

#[derive(Component, Deref, DerefMut)]
pub struct GoldValue(f32);

#[derive(Component, Deref, DerefMut)]
pub struct Quantity(u32);

#[derive(Component)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

#[derive(Component)]
pub struct Eatable;

#[derive(Component)]
pub struct Plantable;

#[derive(Component)]
pub struct Sellable;

