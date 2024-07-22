use crate::prelude::*;
use sickle_ui::prelude::*;

mod autoplay;
mod news;
mod player_turn;

#[allow(unused)]
pub mod prelude {
    pub use super::{
        ControlState, GameFlowPlugin, Season, SeasonOrder,
    };
}

#[derive(SubStates, Debug, Default, Hash, Clone, PartialEq, Eq)]
#[source(GameState = GameState::Playing)]
pub enum ControlState {
    #[default]
    News,
    PlayerTurn,
    Autoplay,
}

#[derive(Resource, Default)]
pub struct GameSession {
    rounds_elapes: u32,
    current_season: Season,
}

pub struct GameFlowPlugin;
impl Plugin for GameFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            autoplay::AutplayPlugin,
            player_turn::PlayerTurnPlugin,
            news::NewsPlugin,
        ));

        app.init_resource::<GameSession>()
            .init_state::<ControlState>()
            .enable_state_scoped_entities::<ControlState>()
            .add_systems(OnEnter(GameState::Playing), new_game);
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Default, Clone, Copy)]
pub enum Season {
    #[default]
    Spring,
    Summer,
    Autum,
    Winter,
}

pub const SeasonOrder: &[Season; 4] = &[
    Season::Spring,
    Season::Summer,
    Season::Autum,
    Season::Winter,
];

fn new_game(
    mut cmd: Commands,
    mut flow: ResMut<NextState<ControlState>>,
) {
    cmd.insert_resource(GameSession::default());
    flow.set(ControlState::PlayerTurn)
}
