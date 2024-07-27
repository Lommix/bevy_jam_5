pub use crate::prelude::*;

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Inventory>();
        app.add_systems(
            First,
            (on_removed, on_added, work_queue).chain(),
        );
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
            cmd.entity(inv.queue).despawn_recursive();
        });
    });
}

fn on_added(
    mut added: Query<&mut Inventory, Added<Inventory>>,
    mut cmd: Commands,
) {
    added.iter_mut().for_each(|mut inventory| {
        if inventory.bag == Entity::PLACEHOLDER {
            inventory.bag = cmd.spawn_empty().id();
        }
        if inventory.queue == Entity::PLACEHOLDER {
            inventory.queue = cmd.spawn_empty().id();
        }
    });
}

fn work_queue(
    mut cmd: Commands,
    mut items: Query<(&mut Quantity, &Item)>,
    inventories: Query<&Inventory>,
    children: Query<&Children>,
) {
    for inventory in inventories.iter() {
        let Ok(content) = children.get(inventory.bag) else {
            continue;
        };

        let Ok(in_queue) = children.get(inventory.queue) else {
            continue;
        };

        for incoming in in_queue.iter() {
            let (in_quant, in_item) = match items.get(*incoming) {
                Ok((quant, item)) => (quant.clone(), item.clone()),
                Err(_) => continue,
            };

            let mut merged = false;
            for existing_item in content.iter() {
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
                cmd.entity(inventory.bag).add_child(*incoming);
            }
        }

        cmd.entity(inventory.queue).clear_children();
    }
}

#[derive(Component, Debug, Reflect, Clone, Copy)]
#[reflect]
pub struct Inventory {
    pub gold: f32,
    pub bag: Entity,
    pub queue: Entity,
}

impl Inventory {
    pub fn from_bag(bag: Entity) -> Self {
        Self {
            gold: 0.,
            bag,
            queue: Entity::PLACEHOLDER,
        }
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            gold: 0.,
            bag: Entity::PLACEHOLDER,
            queue: Entity::PLACEHOLDER,
        }
    }
}
