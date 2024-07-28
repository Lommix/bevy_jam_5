#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::render::texture::ImageSamplerDescriptor;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod app_state;
mod assets;
mod common;
mod flow;
mod items;
mod render;
mod ron_loader;
mod rval;
mod ui;
mod village;
mod widgets;

#[allow(unused)]
pub mod prelude {
    pub use super::app_state::AppState;
    pub use super::assets::*;
    pub use super::common::prelude::*;
    pub use super::flow::prelude::*;
    pub use super::items::prelude::*;
    pub use super::render::prelude::*;
    pub use super::ron_loader::*;
    pub use super::rval::prelude::*;
    pub use super::ui::prelude::*;
    pub use super::village::prelude::*;
    pub use super::widgets::prelude::*;
    pub use bevy::prelude::*;
}

#[rustfmt::skip]
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
            ..default()
            })
            .set(ImagePlugin{
                    default_sampler: ImageSamplerDescriptor::nearest()
            })
        )
        .add_plugins((
            sickle_ui::SickleUiPlugin,
            avian2d::PhysicsPlugins::default(),
            WorldInspectorPlugin::default(),
            bevy_aseprite_ultra::BevySprityPlugin,
        ))
        .add_plugins((
            widgets::UiPlugin,
            ui::MenuPlugin,
            app_state::GameStatePlugin,
            flow::GameFlowPlugin,
            village::VillagePlugin,
            items::ItemPlugin,
            render::RenderPlugin,
            common::CommonPlugin,
        ))
        .run();
}
