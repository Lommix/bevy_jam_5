use crate::prelude::*;

pub struct FoodCostPlugin;
impl Plugin for FoodCostPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            First,
            drain_food
                .run_if(on_event::<SeasonShiftEvent>())
                .in_set(GameSysSets::InGame),
        );
    }
}

fn drain_food(
    mut cmd: Commands,
    mut village: Query<(&mut Village, &Inventory)>,
    bag: Query<&Children>,
    mut foods: Query<&mut Quantity, With<Eatable>>,
) {
    village.iter_mut().for_each(|(mut village, inventory)| {
        let mut to_feed = village.villager_count;

        let Ok(inventory_content) = bag.get(inventory.bag) else {
            return;
        };

        for item_ent in inventory_content.iter() {
            if to_feed == 0 {
                break;
            }

            let Ok(mut quant) = foods.get_mut(*item_ent) else {
                continue;
            };

            let new_quant = **quant - to_feed;
            to_feed = new_quant.min(0).abs();

            if new_quant < 0 {
                cmd.entity(*item_ent).despawn_recursive();
                continue;
            }

            **quant = new_quant;
        }

        if to_feed > 0 {
            village.villager_count -= to_feed;
            village.villager_count = village.villager_count.max(0);
        }
    });
}
