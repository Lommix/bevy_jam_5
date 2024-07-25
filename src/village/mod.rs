pub use crate::prelude::*;
pub use bevy::prelude::*;

mod death;
mod food_cost;
mod house;
mod map;
mod plant;
mod workorder;

pub mod prelude {
    pub use super::house::House;
    pub use super::map::prelude::*;
    pub use super::workorder::prelude::*;
    pub use super::{Village, VillageBundle};
}

pub struct VillagePlugin;
impl Plugin for VillagePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            food_cost::FoodCostPlugin,
            map::VillageTileamap,
            plant::PlantPlugin,
            workorder::WorkOrderPlugin,
        ));
    }
}

#[derive(Bundle)]
pub struct VillageBundle {
    pub village: Village,
    pub inventory: Inventory,
}

#[derive(Component, Debug, Reflect)]
#[reflect]
pub struct Village {
    pub villager_count: i32,
    pub villager_busy: i32,
}
