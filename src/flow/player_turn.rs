use crate::prelude::*;
use sickle_ui::prelude::*;

use bevy::prelude::*;
pub struct PlayerTurnPlugin;
impl Plugin for PlayerTurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ControlState::PlayerTurn),
            start_turn,
        );
    }
}

fn start_turn(mut cmd: Commands) {
    cmd.ui_builder(UiRoot)
        .container(NodeBundle::default(), |builder| {
            builder
                .style()
                .width(Val::Percent(100.))
                .height(Val::Percent(100.))
                .display(Display::Flex)
                .justify_content(JustifyContent::Center)
                .align_items(AlignItems::End);

            builder
                .button("End turn", Size::Medium, ())
                .entity_commands()
                .observe(end_turn);
        })
        .insert(StateScoped(ControlState::PlayerTurn));
}

fn end_turn(
    _trigger: Trigger<ButtonClicked>,
    mut flow: ResMut<NextState<ControlState>>,
) {
    flow.set(ControlState::Autoplay)
}
