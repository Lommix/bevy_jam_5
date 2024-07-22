#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod assets;
mod flow;
mod items;
mod menu;
mod state;
mod ui;
mod village;

#[allow(unused)]
pub mod prelude {
    pub use super::assets::*;
    pub use super::flow::prelude::*;
    pub use super::items::prelude::*;
    pub use super::state::GameState;
    pub use super::ui::prelude::*;
    pub use super::village::prelude::*;
    pub use bevy::prelude::*;
}

#[rustfmt::skip]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins((
            sickle_ui::SickleUiPlugin,
            avian2d::PhysicsPlugins::default(),
            WorldInspectorPlugin::default(),
        ))
        .add_plugins((
            ui::UiPlugin,
            menu::MenuPlugin,
            state::GameStatePlugin,
            flow::GameFlowPlugin,
            village::VillagePlugin,
            items::InventoryPlugin,
        ))
        .run();
}
