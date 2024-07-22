use crate::prelude::*;
use bevy::prelude::*;
use sickle_ui::prelude::*;

pub struct NewsPlugin;
impl Plugin for NewsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ControlState::News), start_news);
        app.add_systems(
            Update,
            news.run_if(in_state(ControlState::News)),
        );
    }
}

fn start_news(mut cmd: Commands) {
    info!("starting news!");
    cmd.ui_builder(UiRoot)
        .container(NodeBundle::default(), |builder| {
            builder
                .button("ok", Size::Medium, ())
                .entity_commands()
                .observe(finish_news);
        })
        .insert(StateScoped(ControlState::News));
}

fn finish_news(
    _trigger: Trigger<ButtonClicked>,
    mut next_state: ResMut<NextState<ControlState>>,
) {
    next_state.set(ControlState::PlayerTurn);
}

fn news(
    mut flow: ResMut<NextState<ControlState>>,
    mut timer: Local<f32>,
    time: Res<Time>,
) {
    *timer += time.delta_seconds();
    if *timer < 30. {
        return;
    }
    *timer = 0.;

    info!("news timeout");
    flow.set(ControlState::PlayerTurn)
}
