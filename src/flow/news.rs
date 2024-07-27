use std::collections::VecDeque;

use crate::prelude::*;
use bevy::prelude::*;
use sickle_ui::prelude::*;

#[allow(unused)]
pub mod prelude {
    pub use super::NewsEvent;
}

pub struct NewsPlugin;
impl Plugin for NewsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewsEvent>();
        app.add_systems(OnEnter(ControlFlow::News), start_news);
        app.add_systems(
            Update,
            news_timeout.run_if(in_state(ControlFlow::News)),
        );
    }
}

#[derive(Event)]
pub struct NewsEvent {
    pub message: String,
}

fn show_news(
    mut cmd: Commands,
    server: Res<AssetServer>,
    window: Query<&Window>,
) {
}

fn start_news(
    mut cmd: Commands,
    // village: Query<(&Village, &GameContext), With<Player>>,
    mut events: EventReader<NewsEvent>,
    ui_query: Query<Entity, With<BottomUi>>,
) {
    info!("starting news!");

    let Ok(ui_ent) = ui_query.get_single() else {
        return;
    };

    cmd.ui_builder(ui_ent)
        .div_centered(|builder| {
            builder
                .dialog(
                    "A new season started, nothing happend so far. Let's plant some crops!",
                )
                .entity_commands()
                .observe(finish_news);
        })
        .insert(StateScoped(ControlFlow::News));
}

fn finish_news(
    _trigger: Trigger<DialogClosed>,
    mut next_state: ResMut<NextState<ControlFlow>>,
) {
    next_state.set(ControlFlow::PlayerTurn);
}

fn news_timeout(
    mut flow: ResMut<NextState<ControlFlow>>,
    mut timer: Local<f32>,
    time: Res<Time>,
) {
    *timer += time.delta_seconds();
    if *timer < 30. {
        return;
    }
    *timer = 0.;

    flow.set(ControlFlow::PlayerTurn)
}
