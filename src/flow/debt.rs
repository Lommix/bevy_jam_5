use crate::prelude::*;

pub struct DebtPlugin;
impl Plugin for DebtPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Season::Spring), on_new_year);
    }
}

fn on_new_year(
    mut events: EventWriter<NewsEvent>,
    mut inventory: Query<
        (&mut Inventory, &GameContext),
        With<Player>,
    >,
    mut next_state: ResMut<NextState<ControlFlow>>,
) {
    let Ok((mut inventory, context)) = inventory.get_single_mut()
    else {
        return;
    };

    inventory.gold -=
        100. * (1. + context.current_year() as f32 * 0.1);

    if inventory.gold < 0. {
        events.send(NewsEvent { message: format!("Oh no! You messed up! You went in depbt to pay taxes.") });
    }

    if inventory.gold < -1000. {
        next_state.set(ControlFlow::Score);
    }
}
