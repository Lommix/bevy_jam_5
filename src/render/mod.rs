use crate::prelude::*;
use bevy::{ecs::query::QueryItem, render::*};
use render_graph::{NodeRunError, RenderGraphContext};
use renderer::RenderContext;

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
    }
}

pub const HIGHLIGHT_IMAGE: Handle<Image> =
    Handle::weak_from_u128(12356415730123899542331);

#[derive(Default, Hash, PartialEq, Eq, Debug, Clone)]
pub struct HighlightLabel;

#[derive(Default, Debug)]
pub struct HighlightNode;

impl render_graph::ViewNode for HighlightNode {
    type ViewQuery = ();
    fn run<'w>(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        view_query: QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        todo!()
    }
}
