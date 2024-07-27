use crate::prelude::*;

mod highlight;
mod targets;

#[allow(unused)]
pub mod prelude {
    pub use super::highlight::prelude::*;
}

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            highlight::HighlightPlugin,
            targets::RenderTargetPlugin,
        ));
    }
}
