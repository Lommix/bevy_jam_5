use crate::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use sickle_ui::prelude::*;

pub struct PlantPlugin;
impl Plugin for PlantPlugin {
    fn build(&self, app: &mut App) {
        app.observe(spawn_plant_order_menu);
    }
}

#[derive(Component)]
pub struct PlantOrder {
    tile: Entity,
    item: Entity,
}

#[derive(Component)]
pub struct PlantOrderPanel;

fn spawn_plant_order_menu(
    //@todo: fix later
    trigger: Trigger<TileClickEvent>,
    mut cmd: Commands,
    bag: Query<&Children>,
    inventory: Query<(&Village, &Inventory)>,
    plantables: Query<(Entity, &Item, &Quantity), With<Plantable>>,
    center_ui: Query<Entity, With<CenterRightUi>>,
    open_panels: Query<Entity, With<PlantOrderPanel>>,
    assets: Res<SpriteAssets>,
) {
    let Ok(center) = center_ui.get_single() else {
        return;
    };

    let Ok((_, inventory)) = inventory.get_single() else {
        return;
    };

    let Ok(items) = bag.get(inventory.bag) else {
        return;
    };

    open_panels
        .iter()
        .for_each(|ent| cmd.entity(ent).despawn_recursive());

    let plantables = items
        .iter()
        .flat_map(|entity| plantables.get(*entity).ok())
        .collect::<Vec<_>>();

    cmd.ui_builder(center)
        .div_centered(|builder| {
            builder.panel_bg(
                PanelConfig::title("Build").with_close(),
                |builder| {
                    builder
                        .style()
                        .flex_direction(FlexDirection::Column)
                        .row_gap(Val::Px(5.))
                        .width(Val::Px(250.));

                    plantables.iter().for_each(
                        |(item_entity, item, _)| {
                            builder
                                .button_item(
                                    assets.icons.clone(),
                                    &item.icon,
                                    &format!("Plant {}", &item.name),
                                )
                                .insert(PlantOrder {
                                    tile: trigger.entity(),
                                    item: *item_entity,
                                })
                                .entity_commands()
                                .observe(plant_seed_work_order);
                        },
                    );
                    if plantables.len() == 0 {
                        builder
                            .text("No seeds to plant", Size::Medium);
                    }
                },
            );
        })
        .insert(PlantOrderPanel)
        .insert(StateScoped(ControlFlow::PlayerTurn));
}

fn plant_seed_work_order(
    trigger: Trigger<ButtonClicked>,
    mut cmd: Commands,
    menues: Query<Entity, With<PlantOrderPanel>>,
    orders: Query<&PlantOrder>,
    sprits: Res<SpriteAssets>,
    mut items: Query<&mut Quantity, With<Item>>,
) {
    let Ok(order) = orders.get(trigger.entity()) else {
        return;
    };

    menues
        .iter()
        .for_each(|ent| cmd.entity(ent).despawn_recursive());

    let outcome = cmd.spawn(carrots(40)).id();

    let field = cmd
        .spawn((
            AsepriteAnimationBundle {
                transform: Transform::from_translation(Vec3::Z),
                aseprite: sprits.field.clone(),
                ..default()
            },
            WorkOrder {
                remaining_turns: 2,
                workforce: 2,
                outcome,
            },
        ))
        .observe(harvest)
        .id();

    _ = items.get_mut(order.item).map(|mut quant| {
        **quant = **quant - 1;
    });

    cmd.entity(order.tile).add_child(field);
}

fn harvest(
    trigger: Trigger<WorkOrderFinished>,
    orders: Query<&WorkOrder>,
    mut inventory: Query<&mut Inventory, With<Player>>,
    mut cmd: Commands,
) {
    let Ok(order) = orders.get(trigger.entity()) else {
        return;
    };

    let Ok(mut inventory) = inventory.get_single_mut() else {
        return;
    };

    inventory.add_item(order.outcome);
    cmd.entity(trigger.entity()).despawn_recursive();
}
