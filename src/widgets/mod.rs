use bevy::prelude::*;

mod aseprite;
mod button;
mod button_item;
mod dialog;
mod div;
mod panel;
mod text;
mod village_hud;

#[allow(unused)]
pub mod prelude {
    pub use super::aseprite::AsepriteExt;
    pub use super::button::{
        ButtonClicked, ButtonWidget, ButtonWidgetExt,
    };
    pub use super::button_item::*;
    pub use super::dialog::{DialogClosed, DialogExt, DialogWidget};
    pub use super::div::{DivCenteredExt, DivExt};
    pub use super::panel::{
        Panel, PanelClosed, PanelConfig, PanelExt,
    };
    pub use super::text::TextExt;
    pub use super::village_hud::{VillageHudExt, VillageHudWidget};
    pub use super::{Size, UiPlugin};
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            panel::PanelWidgetPlugin,
            button::ButtonWidgetPlugin,
            village_hud::VillageHudPlugin,
            dialog::DialogWidgetPlugin,
        ));
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Reflect)]
#[reflect]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
}

impl Size {
    fn val(&self) -> f32 {
        match self {
            Size::Small => 15.,
            Size::Medium => 20.,
            Size::Large => 25.,
        }
    }
}
