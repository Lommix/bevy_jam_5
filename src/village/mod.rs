pub use crate::prelude::*;
pub use bevy::prelude::*;

mod workorder;

pub mod prelude {
    pub use super::{Village, VillageBundle};
}

pub struct VillagePlugin;
impl Plugin for VillagePlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Bundle)]
pub struct VillageBundle {
    village: Village,
    inventory: Inventory,
}

#[derive(Component, Debug, Reflect)]
#[reflect]
pub struct Village {
    villager_count: u32,
    gold: i32,
    food: i32,
}
