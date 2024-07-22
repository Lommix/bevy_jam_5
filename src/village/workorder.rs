pub use crate::prelude::*;

pub struct WorkOrderPlugin;
impl Plugin for WorkOrderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Village>()
            .register_type::<WorkOrder>()
            .add_systems(
                OnExit(ControlState::Autoplay),
                progress_workorder,
            );
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect]
pub struct WorkOrder {
    pub remaining_turns: u32,
    pub workforce: u32,
}

#[derive(Event, Deref, DerefMut)]
pub struct WorkOrderFinishedEvent(Entity);

pub(crate) fn progress_workorder(
    mut events: EventWriter<WorkOrderFinishedEvent>,
    mut query: Query<(Entity, &mut WorkOrder)>,
) {
    query.iter_mut().for_each(|(entity, mut order)| {
        if order.remaining_turns - 1 == 0 {
            events.send(WorkOrderFinishedEvent(entity));
            return;
        }
        order.remaining_turns =
            order.remaining_turns.checked_sub(1).unwrap_or_default();
    });

    info!("workorders progressed");
}
