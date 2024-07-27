pub use crate::prelude::*;
use crate::ron_asset_loader;
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
                OnExit(ControlFlow::Autoplay),
                progress_workorder,
            )
            .observe(finish_workorder)
            .add_systems(Update, update_workforce);
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
    mut query: Query<(Entity, &mut Working, &Handle<WorkOrder>)>,
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

fn finish_workorder(
    trigger: Trigger<WorkOrderFinished>,
    mut cmd: Commands,
    orders: Query<(&Handle<WorkOrder>, &TargetInventory)>,
    work: Res<Assets<WorkOrder>>,
    items: Res<Assets<ItemAsset>>,
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
        let Some(item) = items.get(&output.item_handle) else {
            warn!("crafting - missing item");
            return;
        };

        let item_id =
            cmd.spawn_item(item, output.quantity as i32).id();
        cmd.entity(inventory.queue).add_child(item_id);
    }
}
