pub use crate::prelude::*;
pub use bevy::prelude::*;

mod build;
mod death;
mod food;
mod map;
mod work;

pub mod prelude {
    pub use super::build::{BuildingAsset, BuildingBundle};
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
        app.add_systems(
            Update,
            update_village_count.in_set(GameSysSets::InGame),
        );
    }
}

fn update_village_count(
    houses: Query<&Handle<BuildingAsset>>,
    mut village: Query<&mut Village>,
    buildings: Res<BuildingAssets>,
) {
    let house_count = houses
        .iter()
        .filter(|h| h.id() == buildings.house.id())
        .count();

    let Ok(mut village) = village.get_single_mut() else {
        return;
    };

    village.villager_count = house_count as i32 * 4;
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
