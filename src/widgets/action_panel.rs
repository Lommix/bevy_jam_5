use crate::prelude::*;
use sickle_ui::prelude::*;

use super::sell_panel::SellPanelExt;

pub mod prelude {
    pub use super::{ActionPanelExt, ActionPanelWidget};
}

pub struct ActionPanelPlugin;
impl Plugin for ActionPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_actions);
    }
}

fn load_actions(
    mut cmd: Commands,
    panels: Query<
        (Entity, &ActionPanelWidget),
        Added<ActionPanelWidget>,
    >,
    player: Query<Entity, With<Player>>,
    children: Query<&Children>,
    buildings: Query<&Handle<BuildingAsset>>,
    assets: Res<Assets<BuildingAsset>>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };

    panels.iter().for_each(|(widget_entity, widget)| {
        let maybe_building = children
            .get(widget.tile)
            .ok()
            .map(|children| {
                children
                    .iter()
                    .map(|child| buildings.get(*child).ok())
                    .flatten()
                    .next()
            })
            .flatten()
            .map(|handle| assets.get(handle))
            .flatten();

        match maybe_building {
            Some(building) => {
                // hey sometimes, time just runs out
                if building.name == "House" {
                    cmd.ui_builder(widget_entity)
                        .inventory_panel(player);
                } else if building.name == "Market" {
                    cmd.ui_builder(widget_entity).sell_panel();
                } else {
                    cmd.ui_builder(widget_entity)
                        .tile_info(widget.tile);
                }
            }
            None => {
                cmd.ui_builder(widget_entity)
                    .build_panel(widget.tile);
            }
        }
    });
}

#[derive(Component)]
pub struct ActionPanelWidget {
    tile: Entity,
}
impl Default for ActionPanelWidget {
    fn default() -> Self {
        Self {
            tile: Entity::PLACEHOLDER,
        }
    }
}

pub trait ActionPanelExt {
    fn action_panel(
        &mut self,
        tile: Entity,
        position: Vec2,
    ) -> UiBuilder<Entity>;
}

impl ActionPanelExt for UiBuilder<'_, Entity> {
    fn action_panel(
        &mut self,
        tile: Entity,
        position: Vec2,
    ) -> UiBuilder<Entity> {
        let mut widget = ActionPanelWidget::default();
        widget.tile = tile;
        self.container((NodeBundle::default(), widget), |panel| {
            panel
                .style()
                .position_type(PositionType::Absolute)
                .absolute_position(position);
        })
    }
}
