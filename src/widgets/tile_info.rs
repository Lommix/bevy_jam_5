use crate::prelude::*;
use sickle_ui::prelude::*;

#[allow(unused)]
pub mod prelude {
    pub use super::{TileInfoExt, TileInfoWidget};
}

pub struct TileInfoPanelPlugin;
impl Plugin for TileInfoPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_valid_actions);
    }
}

#[derive(Component)]
pub struct TileInfoWidget {
    pub tile: Entity,
}

#[derive(Component, PartialEq, Eq)]
pub enum TileAction {
    Sell,
    SetOrder(Handle<WorkOrder>),
}

fn load_valid_actions(
    mut cmd: Commands,
    info_panels: Query<(Entity, &TileInfoWidget), Without<Children>>,
    children: Query<&Children>,
    buildings: Query<&Handle<BuildingAsset>>,

    order_assets: Res<Assets<WorkOrder>>,
    building_assets: Res<Assets<BuildingAsset>>,
) {
    info_panels.iter().for_each(|(entity, widget)| {
        // find first building
        let Some(building) = children
            .get(widget.tile)
            .ok()
            .map(|children| {
                children
                    .iter()
                    .flat_map(|child| buildings.get(*child).ok())
                    .next()
            })
            .flatten()
            .map(|handle| building_assets.get(handle))
            .flatten()
        else {
            return;
        };

        building.produce_handles.iter().for_each(|handle| {
            let Some(order) = order_assets.get(handle) else {
                return;
            };

            cmd.ui_builder(entity).button(|button| {
                button.insert(TileAction::SetOrder(handle.clone()));
                button.text(&order.name, Size::Small);
            });
        });
    });
}

fn on_action_pressed() {}

pub trait TileInfoExt {
    fn tile_info(&mut self, tile: Entity) -> UiBuilder<Entity>;
}

impl TileInfoExt for UiBuilder<'_, Entity> {
    fn tile_info(&mut self, tile: Entity) -> UiBuilder<Entity> {
        self.panel_bg(PanelConfig::title("Info"), |panel| {
            panel.insert(TileInfoWidget { tile });
            panel.text("Something about this tile", Size::Small);
        })
    }
}
