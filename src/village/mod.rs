pub use crate::prelude::*;
pub use bevy::prelude::*;

mod build;
mod death;
mod food;
mod map;
mod work;

pub mod prelude {
    pub use super::build::{Building, BuildingAsset, BuildingBundle};
    pub use super::map::prelude::*;
    pub use super::work::prelude::*;
    pub use super::{Village, VillageBundle};
}

pub struct VillagePlugin;
impl Plugin for VillagePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            food::FoodCostPlugin,
            map::VillageTileamap,
            build::BuildPlugin,
            work::WorkOrderPlugin,
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
