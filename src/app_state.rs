use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(States, Debug, Default, Hash, Clone, PartialEq, Eq)]
pub enum AppState {
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
        app.init_state::<AppState>()
            .enable_state_scoped_entities::<AppState>()
            .add_loading_state(
                LoadingState::new(AppState::Startup)
                    .continue_to_state(AppState::Menu)
                    .load_collection::<SpriteAssets>()
                    .load_collection::<AudioAssets>(),
            );
    }
}
