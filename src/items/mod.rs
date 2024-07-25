pub use crate::prelude::*;

mod foods;
mod inventory;

pub mod prelude {
    pub use super::foods::*;
    pub use super::{
        Eatable, GoldValue, Inventory, InventoryPlugin, Item,
        ItemBundle, Plantable, Quantity,
    };
}

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, (on_removed, add_items));
    }
}

fn on_removed(
    inventories: Query<&Inventory>,
    mut removed: RemovedComponents<Inventory>,
    mut cmd: Commands,
) {
    removed.read().for_each(|entity| {
        _ = inventories.get(entity).map(|inv| {
            warn!("despawning items, nobody wants");
            cmd.entity(inv.bag).despawn_recursive();
        });
    });
}

fn add_items(
    mut cmd: Commands,
    mut inventories: Query<&mut Inventory>,
    mut items: Query<(&mut Quantity, &Item)>,
    bag: Query<&Children>,
) {
    for mut inventory in inventories.iter_mut() {
        let Ok(bag) = bag.get(inventory.bag) else {
            inventory.bag = cmd
                .spawn_empty()
                .push_children(inventory.incoming.as_slice())
                .id();
            return;
        };

        for in_entity in inventory.incoming.iter() {
            let (in_quant, in_item) = match items.get(*in_entity) {
                Ok((quant, item)) => (quant.clone(), item.clone()),
                Err(_) => continue,
            };

            let mut merged = false;
            for existing_item in bag.iter() {
                let Ok((mut quant, item)) =
                    items.get_mut(*existing_item)
                else {
                    continue;
                };

                if item.name == in_item.name {
                    **quant = **quant + *in_quant;
                    merged = true;
                }
            }

            // add
            if !merged {
                cmd.entity(inventory.bag).add_child(*in_entity);
            }
        }

        inventory.incoming.clear();
    }
}

#[derive(Component, Debug)]
pub struct Inventory {
    pub gold: f32,
    pub bag: Entity,
    incoming: Vec<Entity>,
}

impl Inventory {
    pub fn add_item(&mut self, item: Entity) -> &mut Self {
        self.incoming.push(item);
        self
    }

    pub fn from_bag(bag: Entity) -> Self {
        Self {
            gold: 0.,
            bag,
            incoming: vec![],
        }
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            gold: 0.,
            bag: Entity::PLACEHOLDER,
            incoming: vec![],
        }
    }
}

#[derive(Bundle)]
pub struct ItemBundle {
    pub item: Item,
    pub value: GoldValue,
    pub rarity: Rarity,
    pub quantity: Quantity,
}

#[derive(Component, Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub icon: String,
}

#[derive(Component, Deref, DerefMut)]
pub struct GoldValue(pub f32);

#[derive(Component, Clone, Deref, DerefMut)]
pub struct Quantity(pub i32);

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
