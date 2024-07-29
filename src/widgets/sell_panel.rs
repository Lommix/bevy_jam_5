use crate::prelude::*;
use sickle_ui::prelude::*;

pub mod prelude {
    use super::{SellPanelExt, SellPanelWidget};
}

pub struct SellPanelPlugin;
impl Plugin for SellPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            load_sell_options.in_set(GameSysSets::InGame),
        );
    }
}

#[derive(Component)]
pub struct SellAction {
    item_handle: Handle<ItemAsset>,
}

fn load_sell_options(
    mut cmd: Commands,
    widget: Query<Entity, Added<SellPanelWidget>>,
    inventory: Query<&Inventory, With<Player>>,
    children: Query<&Children>,
    handles: Query<(&Handle<ItemAsset>, &Quantity)>,
    items: Res<Assets<ItemAsset>>,
    sprites: Res<SpriteAssets>,
) {
    widget.iter().for_each(|widget_entity| {
        let Ok(inventory) = inventory.get_single() else {
            return;
        };
        let Ok(stored_items) = children.get(inventory.bag) else {
            return;
        };
        for item_entity in stored_items.iter() {
            let Ok((handle, quant)) = handles.get(*item_entity)
            else {
                continue;
            };

            let Some(item) = items.get(handle) else {
                continue;
            };

            if item.value <= 0.1 {
                continue;
            }

            cmd.ui_builder(widget_entity)
                .button(|button| {
                    button
                        .style()
                        .justify_content(JustifyContent::SpaceBetween)
                        .column_gap(Val::Px(10.))
                        .align_items(AlignItems::Center)
                        .padding(UiRect::axes(
                            Val::Px(15.),
                            Val::Px(5.),
                        ));

                    button.ase_image(
                        sprites.icons.clone(),
                        &item.icon,
                        |img| {
                            img.style()
                                .width(Val::Px(24.))
                                .height(Val::Px(24.));
                        },
                    );

                    button.text(
                        &format!("Sell {} {}", **quant, item.name),
                        Size::Medium,
                    );
                })
                .insert(SellAction {
                    item_handle: handle.clone(),
                })
                .entity_commands()
                .observe(sell);
        }
    });
}

fn sell(
    trigger: Trigger<ButtonClicked>,
    sell_buttons: Query<&SellAction>,
    mut cmd: Commands,
    mut inventory: Query<&mut Inventory, With<Player>>,
    items: Query<(&Handle<ItemAsset>, &Quantity)>,
    assets: Res<Assets<ItemAsset>>,
    children: Query<&Children>,
) {
    let Ok(action) = sell_buttons.get(trigger.entity()) else {
        return;
    };

    let Ok(mut inventory) = inventory.get_single_mut() else {
        return;
    };

    let Ok(children) = children.get(inventory.bag) else {
        return;
    };

    for item_entity in children.iter() {
        let Ok((handle, quant)) = items.get(*item_entity) else {
            continue;
        };

        if action.item_handle.id() != handle.id() {
            continue;
        };

        let Some(item) = assets.get(handle) else {
            continue;
        };

        inventory.gold += item.value * **quant as f32;
        cmd.entity(*item_entity).despawn_recursive();
    }

    cmd.entity(trigger.entity()).despawn_recursive();
}

#[derive(Component)]
pub struct SellPanelWidget;

pub trait SellPanelExt {
    fn sell_panel(&mut self) -> UiBuilder<Entity>;
}

impl SellPanelExt for UiBuilder<'_, Entity> {
    fn sell_panel(&mut self) -> UiBuilder<Entity> {
        self.panel_bg(
            PanelConfig::title("Market").with_close(),
            |panel| {
                panel
                    .style()
                    .flex_direction(FlexDirection::Column)
                    .min_width(Val::Px(100.))
                    .min_height(Val::Px(200.))
                    .column_gap(Val::Px(15.));

                panel.insert(SellPanelWidget);
            },
        )
    }
}
