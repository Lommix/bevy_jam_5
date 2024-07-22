use bevy::prelude::*;

mod button;

#[allow(unused)]
pub mod prelude {
    pub use super::button::{
        ButtonClicked, ButtonWidget, ButtonWidgetExt,
    };
    pub use super::{Size, UiPlugin};
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((button::ButtonWidgetPlugin,));
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
