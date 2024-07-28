use std::time::Duration;

pub use crate::prelude::*;
use crate::ron_asset_loader;
use bevy_aseprite_ultra::prelude::*;
use bevy_tweening::*;
use lens::{TransformPositionLens, TransformScaleLens};

pub mod prelude {
    pub use super::{
        LoopOrder, TargetInventory, WorkOrder, WorkOrderBundle,
        WorkOrderFinished, Working,
    };
}

pub struct WorkOrderPlugin;
impl Plugin for WorkOrderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Village>()
            .add_plugins(WorkOrderAssetPlugin)
            .add_event::<WorkOrderFinished>()
            .add_systems(
                OnEnter(ControlFlow::Autoplay),
                (progress_workorder, on_missing_spawn_float),
            )
            .observe(finish_workorder)
            .observe(sub_item_cost)
            .observe(on_produce_particles)
            .add_systems(
                Update,
                (update_workforce, update_has_items),
            );
    }
}

ron_asset_loader!(
    WorkOrderAssetPlugin,
    WorkOrderAssetLoader,
    WorkOrder,
    &["work.ron"],
    ;inputs -> (item -> item_handle)
    ;outputs -> (item -> item_handle)
);

#[derive(Reflect, serde::Deserialize, serde::Serialize)]
#[reflect]
pub struct ItemSlot {
    pub quantity: u32,
    pub item: String,
    #[serde(skip)]
    pub item_handle: Handle<ItemAsset>,
}

#[derive(Reflect, Debug, serde::Serialize, serde::Deserialize)]
#[reflect]
pub enum Interval {
    Seasons(Vec<Season>),
    Step(u32),
}

#[derive(Asset, TypePath, serde::Deserialize, serde::Serialize)]
pub struct WorkOrder {
    pub name: String,
    pub description: String,

    pub interval: Interval,
    pub workforce: u32,
    pub gold_cost: Option<f32>,

    pub inputs: Vec<ItemSlot>,
    pub outputs: Vec<ItemSlot>,
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct Working(pub u32);

#[derive(Event)]
pub struct WorkOrderFinished;

#[derive(Component)]
pub struct LoopOrder;

#[derive(Component, Deref, DerefMut)]
pub struct TargetInventory(pub Entity);

pub(crate) fn update_workforce(
    mut village: Query<&mut Village, With<Player>>,
    orders: Query<&Handle<WorkOrder>>,
    work: Res<Assets<WorkOrder>>,
) {
    let Ok(mut village) = village.get_single_mut() else {
        return;
    };

    village.villager_busy = orders
        .iter()
        .flat_map(|handle| work.get(handle))
        .map(|order| order.workforce)
        .sum::<u32>() as i32;
}

#[derive(Bundle)]
pub struct WorkOrderBundle {
    pub order: Handle<WorkOrder>,
    pub target: TargetInventory,
    pub working: Working,
}

pub(crate) fn progress_workorder(
    mut cmd: Commands,
    mut query: Query<
        (Entity, &mut Working, &Handle<WorkOrder>),
        With<HasItemsToCraft>,
    >,
    season: Res<State<Season>>,
    work: Res<Assets<WorkOrder>>,
    repeat: Query<&LoopOrder>,
) {
    query.iter_mut().for_each(|(entity, mut working, handle)| {
        let Some(order) = work.get(handle) else {
            warn!("crafting - unknown order");
            return;
        };

        let finished = match &order.interval {
            Interval::Seasons(allowed) => {
                allowed.contains(season.get())
            }
            Interval::Step(max_steps) => {
                **working += 1;
                **working >= *max_steps
            }
        };

        if finished {
            info!("finished work order");
            cmd.trigger_targets(WorkOrderFinished, entity);
            match repeat.get(entity) {
                Ok(_) => {
                    **working = 0;
                }
                Err(_) => {
                    cmd.entity(entity).remove::<Working>();
                }
            };
        }
    });
}

#[derive(Component)]
pub struct HasItemsToCraft;

fn update_has_items(
    mut cmd: Commands,
    query: Query<(Entity, &Handle<WorkOrder>, &TargetInventory)>,
    inventories: Query<&Inventory>,
    children: Query<&Children>,
    items: Query<(&Handle<ItemAsset>, &Quantity)>,
    orders: Res<Assets<WorkOrder>>,
) {
    query.iter().for_each(|(entity, handle, target)| {
        let Some(order) = orders.get(handle) else {
            return;
        };

        let Ok(inventory) = inventories.get(**target) else {
            return;
        };

        let Some(owned_items) =
            children.get(inventory.bag).ok().map(|children| {
                children
                    .iter()
                    .flat_map(|child| items.get(*child))
                    .collect::<Vec<_>>()
            })
        else {
            return;
        };

        let has_items = order.inputs.iter().all(|slot| {
            for (handle, quant) in owned_items.iter() {
                if slot.item_handle.id() != handle.id() {
                    continue;
                }

                if ***quant >= slot.quantity as i32 {
                    return true;
                }
            }

            return false;
        });

        if has_items {
            cmd.entity(entity).insert(HasItemsToCraft);
        } else {
            cmd.entity(entity).remove::<HasItemsToCraft>();
        }
    })
}

fn sub_item_cost(
    trigger: Trigger<WorkOrderFinished>,
    inventories: Query<&Inventory>,
    children: Query<&Children>,
    orders: Query<(&Handle<WorkOrder>, &TargetInventory)>,
    work: Res<Assets<WorkOrder>>,
    mut item_entities: Query<(&mut Quantity, &Handle<ItemAsset>)>,
) {
    let Ok((order_handle, target)) = orders.get(trigger.entity())
    else {
        return;
    };

    let Some(order) = work.get(order_handle) else {
        return;
    };

    let Ok(inventory) = inventories.get(**target) else {
        warn!("target does no longer exists");
        return;
    };

    let Some(owned_items_entities) = children
        .get(inventory.bag)
        .ok()
        .map(|children| children.iter().collect::<Vec<_>>())
    else {
        return;
    };

    for slot in order.inputs.iter() {
        for ent in owned_items_entities.iter() {
            let Ok((mut quant, handle)) =
                item_entities.get_mut(**ent)
            else {
                continue;
            };

            if slot.item_handle.id() != handle.id() {
                continue;
            }

            info!(
                "reduced items ({:?}) by {}",
                handle, slot.quantity
            );
            **quant -= slot.quantity as i32;
        }
    }
}

fn finish_workorder(
    trigger: Trigger<WorkOrderFinished>,
    mut cmd: Commands,
    orders: Query<(&Handle<WorkOrder>, &TargetInventory)>,
    work: Res<Assets<WorkOrder>>,
    inventories: Query<&Inventory>,
) {
    let Ok((order_handle, target)) = orders.get(trigger.entity())
    else {
        return;
    };

    let Some(order) = work.get(order_handle) else {
        return;
    };

    let Ok(inventory) = inventories.get(**target) else {
        warn!("target does no longer exists");
        return;
    };

    for output in order.outputs.iter() {
        let item_id = cmd
            .spawn(ItemBundle {
                item: output.item_handle.clone(),
                quantity: Quantity(output.quantity as i32),
            })
            .id();
        cmd.entity(inventory.queue).add_child(item_id);
    }
}

fn on_missing_spawn_float(
    query: Query<
        &Parent,
        (Without<HasItemsToCraft>, With<Handle<WorkOrder>>),
    >,
    fonts: Res<FontAssets>,
    mut cmd: Commands,
) {
    query.iter().for_each(|parent| {
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs_f32(0.5),
            TransformPositionLens {
                start: Vec3::Z * 20.,
                end: Vec3::Z * 20. + Vec3::new(0., 30., 0.),
            },
        );
        let text = cmd
            .spawn((
                Animator::new(tween),
                Lifetime::seconds(0.5),
                Text2dBundle {
                    text: Text::from_section(
                        "No Items",
                        TextStyle {
                            font: fonts.font.clone(),
                            font_size: 14.,
                            ..default()
                        },
                    ),
                    ..default()
                },
            ))
            .id();

        cmd.entity(parent.get()).add_child(text);
    })
}

fn on_produce_particles(
    trigger: Trigger<WorkOrderFinished>,
    mut cmd: Commands,
    producers: Query<(&GlobalTransform, &Handle<WorkOrder>)>,
    items: Res<Assets<ItemAsset>>,
    works: Res<Assets<WorkOrder>>,
    sprites: Res<SpriteAssets>,
) {
    let Ok((postion, order_handle)) =
        producers.get(trigger.entity()).map(|(t, handle)| {
            (t.translation().truncate().extend(100.), handle)
        })
    else {
        return;
    };

    let Some(order) = works.get(order_handle) else {
        return;
    };

    let Some(output) = order
        .outputs
        .iter()
        .next()
        .map(|slot| items.get(&slot.item_handle))
        .flatten()
    else {
        return;
    };

    let tween = Tween::new(
        EaseFunction::BounceOut,
        Duration::from_secs_f32(0.5),
        TransformScaleLens {
            start: Vec3::ZERO,
            end: Vec3::ONE * 2.,
        },
    )
    .then(Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_secs_f32(0.5),
        TransformPositionLens {
            start: postion,
            end: postion + Vec3::new(0., 60., 0.),
        },
    ));
    cmd.spawn((
        AsepriteSliceBundle {
            transform: Transform::from_translation(postion),
            slice: output.icon.as_str().into(),
            aseprite: sprites.icons.clone(),
            ..default()
        },
        Lifetime::seconds(1.),
        Animator::new(tween),
    ));
}
