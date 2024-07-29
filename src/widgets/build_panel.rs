use crate::prelude::*;
use sickle_ui::prelude::*;

pub struct BuildPanelPlugin;
impl Plugin for BuildPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BuildPanelClose>();
        app.add_event::<BuildStarted>();
        app.observe(sub_item_cost);
        app.add_systems(
            Update,
            update_options.in_set(GameSysSets::InGame),
        );
    }
}

fn update_options(
    mut cmd: Commands,
    wigets: Query<
        (Entity, &BuildPanel),
        (With<BuildPanel>, Added<BuildPanel>),
    >,
    village: Query<
        (&Village, &Inventory, &GameContext),
        With<Player>,
    >,
    building_assets: Res<Assets<BuildingAsset>>,
    buildings: Res<BuildingAssets>,
    children: Query<&Children>,
    items: Query<(&Handle<ItemAsset>, &Quantity)>,
    item_assets: Res<Assets<ItemAsset>>,
    sprites: Res<SpriteAssets>,
) {
    let Ok((village, inventory, _)) = village.get_single() else {
        return;
    };

    let Ok(item_entities) = children.get(inventory.bag) else {
        return;
    };

    let free_villager =
        village.villager_count - village.villager_busy;

    for (panel_ent, panel) in wigets.iter() {
        // spawn button
        buildings.iter().for_each(|handle| {
            let Some(building) = building_assets.get(&handle) else {
                return;
            };

            if cmd.get_entity(panel_ent).is_none() {
                return;
            };

            let has_items = building.item_cost.iter().all(|slot| {
                inventory.in_stock(
                    &slot.item_handle,
                    slot.quantity as i32,
                    &children,
                    &items,
                )
            });

            let has_gold = building
                .build_cost
                .map(|cost| cost <= inventory.gold)
                .unwrap_or(true);

            cmd.ui_builder(panel_ent).button(|button| {
                button.insert(BuildOrder {
                    panel: panel_ent,
                    tile: panel.tile,
                    building: handle.clone(),
                });

                button
                    .style()
                    .row_gap(Val::Px(5.))
                    .max_width(Val::Px(300.))
                    .justify_content(JustifyContent::SpaceBetween)
                    .align_items(AlignItems::Center)
                    .width(Val::Px(300.))
                    .background_color(COLOR_INACTIVE);

                button
                    .text(&building.name, Size::Medium)
                    .style()
                    .width(Val::Percent(65.));

                button.div(|div| {
                    div.style()
                        .width(Val::Percent(35.))
                        .justify_content(
                            JustifyContent::SpaceBetween,
                        );

                    if let Some(gold_cost) = building.build_cost {
                        div.div(|div| {
                            div.style()
                                .column_gap(Val::Px(5.))
                                .flex_direction(FlexDirection::Row);

                            div.text(
                                &format!("{}", gold_cost),
                                Size::Small,
                            );

                            div.ase_image(
                                sprites.icons.clone(),
                                "gold",
                                |img| {
                                    img.style()
                                        .height(Val::Px(16.))
                                        .width(Val::Px(16.));
                                },
                            );
                        });
                    }

                    for slot in building.item_cost.iter() {
                        let Some(item) =
                            item_assets.get(&slot.item_handle)
                        else {
                            continue;
                        };

                        div.div(|div| {
                            div.style()
                                .column_gap(Val::Px(5.))
                                .flex_direction(FlexDirection::Row);

                            div.text(
                                &format!("{}", slot.quantity),
                                Size::Small,
                            );

                            div.ase_image(
                                sprites.icons.clone(),
                                &item.icon,
                                |img| {
                                    img.style()
                                        .height(Val::Px(16.))
                                        .width(Val::Px(16.));
                                },
                            );
                        });
                    }
                });

                if has_items
                    && has_gold
                    && free_villager >= building.workforce
                {
                    button.style().background_color(COLOR_ACTIVE);
                    button.entity_commands().observe(build_order);
                }
            });
        })
    }
}

#[derive(Component)]
pub struct BuildOrder {
    panel: Entity,
    tile: Entity,
    building: Handle<BuildingAsset>,
}

#[derive(Event)]
pub struct BuildPanelClose;

#[derive(Component)]
pub struct BuildPanel {
    tile: Entity,
}

impl Default for BuildPanel {
    fn default() -> Self {
        Self {
            tile: Entity::PLACEHOLDER,
        }
    }
}

pub trait BuildPanelExt {
    fn build_panel(&mut self, tile: Entity) -> UiBuilder<Entity>;
}

impl BuildPanelExt for UiBuilder<'_, Entity> {
    fn build_panel(&mut self, tile: Entity) -> UiBuilder<Entity> {
        let mut widget = BuildPanel::default();
        widget.tile = tile;

        let mut out = self.panel_bg(
            PanelConfig::title("Build").with_close(),
            |panel| {
                panel
                    .style()
                    .flex_direction(FlexDirection::Column)
                    .row_gap(Val::Px(5.));
            },
        );
        out.insert(widget);
        out
    }
}

fn build_order(
    trigger: Trigger<ButtonClicked>,
    mut cmd: Commands,
    build_orders: Query<&BuildOrder>,
) {
    let Ok(order) = build_orders.get(trigger.entity()) else {
        return;
    };

    let building = cmd
        .spawn(BuildingBundle {
            building: order.building.clone(),
            ..default()
        })
        .id();

    cmd.trigger(BuildStarted(order.building.clone()));

    cmd.entity(order.tile).add_child(building);
    cmd.trigger_targets(PanelClosed, order.panel);
    cmd.entity(order.panel).despawn_recursive();
}

#[derive(Event)]
pub struct BuildStarted(Handle<BuildingAsset>);

fn sub_item_cost(
    trigger: Trigger<BuildStarted>,
    inventories: Query<&Inventory, With<Player>>,
    children: Query<&Children>,
    buildings: Res<Assets<BuildingAsset>>,
    mut item_entities: Query<(&mut Quantity, &Handle<ItemAsset>)>,
) {
    let Some(bulding) = buildings.get(&trigger.event().0) else {
        return;
    };

    let Ok(inventory) = inventories.get_single() else {
        return;
    };

    let Some(owned_items_entities) = children
        .get(inventory.bag)
        .ok()
        .map(|children| children.iter().collect::<Vec<_>>())
    else {
        return;
    };

    for slot in bulding.item_cost.iter() {
        for ent in owned_items_entities.iter() {
            let Ok((mut quant, handle)) =
                item_entities.get_mut(**ent)
            else {
                continue;
            };

            if slot.item_handle.id() != handle.id() {
                continue;
            }

            **quant -= slot.quantity as i32;
        }
    }
}
