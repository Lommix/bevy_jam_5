use crate::prelude::*;

pub struct DebtPlugin;
impl Plugin for DebtPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Season::Spring), on_new_year);
    }
}

fn on_new_year(
    mut cmd: Commands,
    mut events: EventWriter<NewsEvent>,
    mut inventory: Query<
        (&mut Inventory, &GameContext),
        With<Player>,
    >,
) {
    let Ok((mut inventory, context)) = inventory.get_single_mut()
    else {
        return;
    };

    inventory.gold -=
        20. * (1. + context.current_year() as f32 * 0.1);

    if inventory.gold < 0. && inventory.gold > -300. {
        events.send(NewsEvent { message: format!("Oh no! You messed up! You went in dept to pay taxes. Better don't let it get to high!") });
    }

    if inventory.gold < -300. {
        events.send(NewsEvent { message: format!("The local lord is very disappointed, make money fast or loose your head!") });
    }

    if inventory.gold < -400. {
        cmd.trigger(GameOverEvent::Debt);
    }
}
