use crate::prelude::*;

use bevy::prelude::*;
pub struct AutplayPlugin;
impl Plugin for AutplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ControlFlow::Autoplay),
            spawn_progress_bar,
        );
        app.add_systems(
            Update,
            progress_autoplay.run_if(in_state(ControlFlow::Autoplay)),
        );
    }
}

pub(crate) fn spawn_progress_bar() {
    info!("starting autoplay!");
}

pub(crate) fn progress_autoplay(
    mut counter: Local<f32>,
    mut next_state: ResMut<NextState<ControlFlow>>,
    time: Res<Time>,
) {
    *counter += time.delta_seconds();
    if *counter < 3. {
        return;
    }

    *counter = 0.;
    info!("finished autoplay!");
    next_state.set(ControlFlow::News);
}
