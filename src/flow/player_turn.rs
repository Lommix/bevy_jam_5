use crate::prelude::*;
use sickle_ui::prelude::*;

pub struct PlayerTurnPlugin;
impl Plugin for PlayerTurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ControlFlow::PlayerTurn), start_turn);
        app.observe(open_inventory);
    }
}

// enable clicking
// make descision
// end turn
fn start_turn(
    mut cmd: Commands,
    query: Query<Entity, With<BottomUi>>,
) {
    let Ok(bottom_ui) = query.get_single() else {
        return;
    };

    cmd.ui_builder(bottom_ui)
        .div_centered(|builder| {
            builder
                .button(|builder| {
                    builder.text("End this turn", Size::Large);
                })
                .entity_commands()
                .observe(end_turn);
        })
        .insert(StateScoped(ControlFlow::PlayerTurn));
}

fn end_turn(
    _trigger: Trigger<ButtonClicked>,
    mut flow: ResMut<NextState<ControlFlow>>,
    mut cmd: Commands,
) {
    cmd.trigger(ClearHighlights);
    flow.set(ControlFlow::Autoplay);
}

fn open_inventory(
    trigger: Trigger<TileClickEvent>,
    ui: Query<Entity, With<CenterMiddleUi>>,
    inventory: Query<Entity, With<Player>>,
    mut cmd: Commands,
    houses: Query<(), With<Building>>,
) {
    let Ok(ui_root) = ui.get_single() else {
        return;
    };

    let Ok(player) = inventory.get_single() else {
        return;
    };

    if houses.get(trigger.entity()).is_ok() {
        info!("house clicked");
        cmd.ui_builder(ui_root).inventory_panel(player);
    }
}
