use crate::prelude::*;
use sickle_ui::prelude::*;

pub struct InventoryPanelPlugin;
impl Plugin for InventoryPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            render_inventory.in_set(GameSysSets::InGame),
        );
    }
}

fn render_inventory(
    mut cmd: Commands,
    widgets: Query<(Entity, &InventoryPanelWidget), Added<InventoryPanelWidget>>,
    inventories: Query<&Inventory>,
    children: Query<&Children>,
    items: Query<(&Item, &Quantity)>,
    assets: Res<SpriteAssets>,
) {
    for (widget_ent, widget) in widgets.iter() {
        let Ok(inventory) = inventories.get(widget.inventory) else {
            continue;
        };

        let Ok(content) = children.get(inventory.bag) else {
            continue;
        };

        for item_ent in content.iter() {
            let Ok((item, quant)) = items.get(*item_ent) else {
                continue;
            };

            cmd.ui_builder(widget_ent).button_item(
                assets.icons.clone(),
                &item.icon,
                &format!("{} {}", **quant, &item.name),
            );
        }
    }
}

#[derive(Component)]
pub struct InventoryPanelWidget {
    inventory: Entity,
}

impl InventoryPanelWidget {}

pub trait InventoryPanelExt {
    fn inventory_panel(
        &mut self,
        inventory: Entity,
    ) -> UiBuilder<Entity>;
}

impl InventoryPanelExt for UiBuilder<'_, Entity> {
    fn inventory_panel(
        &mut self,
        inventory: Entity,
    ) -> UiBuilder<Entity> {
        self.panel_bg(
            PanelConfig::title("Inventory").with_close(),
            |panel| {
                panel
                    .style()
                    .flex_direction(FlexDirection::Column)
                    .column_gap(Val::Px(5.));

                panel.insert(InventoryPanelWidget { inventory });
            },
        )
    }
}
