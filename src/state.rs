use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(States, Debug, Default, Hash, Clone, PartialEq, Eq)]
pub enum GameState {
    #[default]
    Startup,
    Menu,
    Loading,
    Playing,
    Score,
}

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .enable_state_scoped_entities::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::Startup)
                    .continue_to_state(GameState::Menu)
                    .load_collection::<AudioAssets>(),
            );
    }
}
