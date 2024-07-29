use crate::prelude::*;

mod debt;
mod game_over;
mod new_game;
mod news;
mod player_turn;
mod seasons;
mod session;

#[allow(unused)]
pub mod prelude {
    pub use super::game_over::GameOverEvent;
    pub use super::news::prelude::*;
    pub use super::seasons::{
        Season, SeasonPlugin, SeasonShiftEvent,
    };
    pub use super::session::{GameContext, GameSessionBundle};
    pub use super::{
        ControlFlow, GameFlowPlugin, GameSysSets, Player,
    };
}

#[derive(SubStates, Debug, Default, Hash, Clone, PartialEq, Eq)]
#[source(AppState = AppState::Playing)]
pub enum ControlFlow {
    #[default]
    Intro,
    Playing,
    Score,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct GameSysLabel;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSysSets {
    InMenu,
    InGame,
}

#[derive(Component)]
pub struct Player;

pub struct GameFlowPlugin;
impl Plugin for GameFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            player_turn::PlayerTurnPlugin,
            news::NewsPlugin,
            seasons::SeasonPlugin,
            game_over::GameOverPlugin,
            debt::DebtPlugin,
        ));

        app.configure_sets(
            Update,
            (
                GameSysSets::InMenu.run_if(in_state(AppState::Menu)),
                GameSysSets::InGame
                    .run_if(in_state(AppState::Playing)),
            ),
        );

        app.init_state::<ControlFlow>()
            .enable_state_scoped_entities::<ControlFlow>()
            .add_systems(
                OnEnter(AppState::Playing),
                new_game::new_game,
            );
    }
}
