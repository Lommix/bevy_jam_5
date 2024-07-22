use crate::prelude::*;
use bevy::prelude::*;
use sickle_ui::prelude::*;

mod menu;
mod overlay;

#[allow(unused)]
pub mod prelude {
    pub use super::overlay::OverlayState;
    pub use super::MenuPlugin;
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            menu::GameMenuPlugin,
            overlay::OverlayPlugin,
        ))
        .add_systems(OnEnter(GameState::Startup), startup)
        .add_systems(OnEnter(GameState::Menu), menu::spawn_menu);
    }
}

fn startup(mut cmd: Commands, mut theme_data: ResMut<ThemeData>) {
    cmd.spawn(Camera2dBundle::default());
    theme_data.active_scheme = Scheme::Light(Contrast::Standard);
}

fn spawn_game(mut cmd: Commands) {
    cmd.ui_builder(UiRoot)
        .container(NodeBundle::default(), |builder| {
            builder
                .button("Exit", Size::Medium, ())
                .entity_commands()
                .observe(end_game);
        })
        .insert(StateScoped(GameState::Playing));
}

fn end_game(
    _trigger: Trigger<ButtonClicked>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    info!("end game");
    next_state.set(GameState::Menu);
}
