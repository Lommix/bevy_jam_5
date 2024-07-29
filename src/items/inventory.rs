use bevy::utils::HashMap;

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
    mut items: Query<(&mut Quantity, &Handle<ItemAsset>)>,
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

        let mut queue_sorted: HashMap<Handle<ItemAsset>, i32> =
            HashMap::new();

        for incoming in in_queue.iter() {
            let (in_quant, in_item) = match items.get(*incoming) {
                Ok((quant, item)) => (quant.clone(), item.clone()),
                Err(_) => continue,
            };

            match queue_sorted.get_mut(&in_item) {
                Some(quant) => {
                    *quant += *in_quant;
                }
                None => {
                    queue_sorted.insert(in_item.clone(), *in_quant);
                }
            };

            cmd.entity(*incoming).despawn_recursive();
        }

        for (handle, q) in queue_sorted.iter() {
            let mut merged = false;
            for existing_item in content.iter() {
                let Ok((mut quant, item)) =
                    items.get_mut(*existing_item)
                else {
                    continue;
                };

                if item.id() == handle.id() {
                    **quant = **quant + *q;
                    merged = true;
                }
            }

            // add
            if !merged {
                let item = cmd
                    .spawn(ItemBundle {
                        item: handle.clone(),
                        quantity: Quantity(*q),
                    })
                    .id();
                cmd.entity(inventory.bag).add_child(item);
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

impl Inventory {
    #[rustfmt::skip]
    pub fn in_stock(
        &self,
        handle: &Handle<ItemAsset>,
        quantity: i32,
        children: &Query<&Children>,
        items: &Query<(&Handle<ItemAsset>, &Quantity)>,
    ) -> bool {
        let available = children
            .get(self.bag)
            .ok()
            .map(|children|{
                children.iter().flat_map(|child|{
                    let ( h, q ) = items.get(*child).ok()?;
                    if h.id() != handle.id(){
                        return None;
                    }
                    return Some(**q);
                }).sum::<i32>()
            }).unwrap_or_default();

        return available >= quantity;
    }
}
