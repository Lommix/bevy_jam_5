use super::targets::RenderTargets;
use crate::prelude::*;
use bevy::{
    core_pipeline::{
        core_2d::graph::{Core2d, Node2d},
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    ecs::{
        component::ComponentId, query::QueryItem,
        world::DeferredWorld,
    },
    render::*,
};
use extract_component::{
    ComponentUniforms, DynamicUniformIndex, ExtractComponent,
    ExtractComponentPlugin, UniformComponentPlugin,
};
use render_asset::RenderAssets;
use render_graph::{
    NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel,
    ViewNodeRunner,
};
use render_resource::{
    binding_types::{sampler, texture_2d, uniform_buffer},
    BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries,
    CachedRenderPipelineId, ColorTargetState, ColorWrites,
    FragmentState, MultisampleState, Operations, PipelineCache,
    PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipelineDescriptor, Sampler, SamplerBindingType,
    SamplerDescriptor, ShaderStages, ShaderType, TextureFormat,
    TextureSampleType,
};
use renderer::{RenderContext, RenderDevice};
use texture::{BevyDefault, GpuImage};
use view::{RenderLayers, ViewTarget};

#[allow(unused)]
pub mod prelude {
    pub use super::{
        ClearHighlights, Highlight, HighlightSettings, RecursiveComps,
    };
}

pub struct HighlightPlugin;
impl Plugin for HighlightPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClearHighlights>()
            .observe(clear_highlights)
            .add_systems(Update, (animate, on_child_added))
            .add_systems(Startup, setup)
            .add_plugins((
                ExtractComponentPlugin::<HighlightSettings>::default(
                ),
                UniformComponentPlugin::<HighlightSettings>::default(
                ),
            ));

        app.world_mut()
            .register_component_hooks::<Highlight>()
            .on_add(on_highlight_add)
            .on_remove(on_highlight_remove);

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<HighlightNode>>(
                Core2d,
                HighlightLabel,
            )
            .add_render_graph_edges(
                Core2d,
                (
                    Node2d::Tonemapping,
                    HighlightLabel,
                    Node2d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.init_resource::<HighlightPipeline>();
    }
}

#[derive(Event)]
pub struct ClearHighlights;

fn clear_highlights(
    _trigger: Trigger<ClearHighlights>,
    mut cmd: Commands,
    highlights: Query<Entity, With<Highlight>>,
) {
    highlights.iter().for_each(|entity| {
        if let Some(mut cmd) = cmd.get_entity(entity) {
            cmd.remove::<Highlight>();
        }
    });
}

fn animate(
    mut query: Query<&mut Transform, With<Highlight>>,
    mut elapsed: Local<f32>,
    time: Res<Time>,
) {
    *elapsed += time.delta_seconds() * 3.;
    query.iter_mut().for_each(|mut transform| {
        transform.scale = Vec3::ONE + elapsed.sin() * 0.05;
    });
}

fn on_child_added(
    mut cmd: Commands,
    query: Query<(Entity, &Parent), Added<Parent>>,
    highlights: Query<(), With<Highlight>>,
) {
    query.iter().for_each(|(entity, parent)| {
        if highlights.get(parent.get()).is_ok() {
            cmd.entity(entity).insert(Highlight);
        }
    });
}

fn on_highlight_add(
    mut world: DeferredWorld,
    entity: Entity,
    _component_id: ComponentId,
) {
    if let Some(mut cmd) = world.commands().get_entity(entity) {
        cmd.try_insert(HIGHTLIGHT);
    }
}
fn on_highlight_remove(
    mut world: DeferredWorld,
    entity: Entity,
    _component_id: ComponentId,
) {
    if let Some(mut cmd) = world.commands().get_entity(entity) {
        cmd.try_insert(NORMAL);
    }
    if let Some(mut transform) = world.get_mut::<Transform>(entity) {
        transform.scale = Vec3::ONE;
    }
}

#[derive(Component, Default)]
pub struct Highlight;

pub trait RecursiveComps {
    fn rec_remove<T: Component + Default>(
        &mut self,
        entity: Entity,
        query: &Query<&Children>,
    );
    fn rec_insert<T: Component + Default>(
        &mut self,
        entity: Entity,
        query: &Query<&Children>,
    );
}

impl RecursiveComps for Commands<'_, '_> {
    fn rec_remove<T: Component + Default>(
        &mut self,
        entity: Entity,
        query: &Query<&Children>,
    ) {
        if let Some(mut cmd) = self.get_entity(entity) {
            cmd.remove::<T>();
        } else {
            return;
        }

        let Ok(children) = query.get(entity) else {
            return;
        };

        for child in children.iter() {
            self.rec_remove::<T>(*child, query);
        }
    }
    fn rec_insert<T: Component + Default>(
        &mut self,
        entity: Entity,
        query: &Query<&Children>,
    ) {
        if let Some(mut cmd) = self.get_entity(entity) {
            cmd.insert(T::default());
        } else {
            return;
        }

        let Ok(children) = query.get(entity) else {
            return;
        };

        for child in children.iter() {
            self.rec_insert::<T>(*child, query);
        }
    }
}

pub const NORMAL: RenderLayers = RenderLayers::layer(0);
pub const HIGHTLIGHT: RenderLayers = RenderLayers::layer(1);

fn setup(mut cmd: Commands, targets: Res<RenderTargets>) {
    cmd.spawn((
        Camera2dBundle {
            transform: Transform::from_scale(Vec3::splat(0.80)),
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::NONE),
                order: 2,
                target: camera::RenderTarget::Image(
                    targets.hight_light_source.clone(),
                ),
                ..default()
            },
            ..default()
        },
        HIGHTLIGHT,
    ));
}

#[derive(Default, RenderLabel, Hash, PartialEq, Eq, Debug, Clone)]
pub struct HighlightLabel;
#[derive(Default, Debug)]
pub struct HighlightNode;
impl render_graph::ViewNode for HighlightNode {
    type ViewQuery = (
        &'static HighlightSettings,
        &'static ViewTarget,
        &'static DynamicUniformIndex<HighlightSettings>,
    );
    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        (_settings, view_target, settings_index): QueryItem<
            'w,
            Self::ViewQuery,
        >,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        let highlight_pipeline =
            world.resource::<HighlightPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipline_id) =
            pipeline_cache.get_render_pipeline(highlight_pipeline.id)
        else {
            warn!("missing pipeline");
            return Ok(());
        };

        let Some(targets) = world.get_resource::<RenderTargets>()
        else {
            warn!("missing pipline");
            return Ok(());
        };

        let gpu_images = world.resource::<RenderAssets<GpuImage>>();

        let Some(highlight_gpu_img) =
            gpu_images.get(&targets.hight_light_source)
        else {
            warn!("missing target");
            return Ok(());
        };

        let settings_uniforms =
            world.resource::<ComponentUniforms<HighlightSettings>>();
        let Some(settings_binding) =
            settings_uniforms.uniforms().binding()
        else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        let bind_group =
            render_context.render_device().create_bind_group(
                Some("hightlight_bind_group".into()),
                &highlight_pipeline.layout,
                &BindGroupEntries::sequential((
                    post_process.source,
                    &highlight_pipeline.sampler,
                    &highlight_gpu_img.texture_view,
                    &highlight_pipeline.sampler,
                    settings_binding.clone(),
                )),
            );

        let mut render_pass = render_context
            .begin_tracked_render_pass(RenderPassDescriptor {
                label: Some("post_process_pass"),
                color_attachments: &[Some(
                    RenderPassColorAttachment {
                        view: post_process.destination,
                        resolve_target: None,
                        ops: Operations::default(),
                    },
                )],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

        render_pass.set_render_pipeline(pipline_id);
        render_pass.set_bind_group(
            0,
            &bind_group,
            &[settings_index.index()],
        );
        render_pass.draw(0..3, 0..1);
        Ok(())
    }
}

#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
pub struct HighlightSettings {
    pub glow_border_thickness: f32,
}
impl Default for HighlightSettings {
    fn default() -> Self {
        Self {
            glow_border_thickness: 5.,
        }
    }
}

#[derive(Resource)]
pub struct HighlightPipeline {
    layout: BindGroupLayout,
    id: CachedRenderPipelineId,
    sampler: Sampler,
}

impl FromWorld for HighlightPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let layout = render_device.create_bind_group_layout(
            "light_pipelin_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float {
                        filterable: true,
                    }),
                    sampler(SamplerBindingType::Filtering),
                    texture_2d(TextureSampleType::Float {
                        filterable: true,
                    }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<HighlightSettings>(true),
                ),
            ),
        );

        let sampler = render_device
            .create_sampler(&SamplerDescriptor::default());

        let shader: Handle<Shader> = world
            .resource::<AssetServer>()
            .load("shader/highlight.wgsl");
        let id = world
            .resource_mut::<PipelineCache>()
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("hightlight_pipline".into()),
                layout: vec![layout.clone()],
                push_constant_ranges: vec![],
                vertex: fullscreen_shader_vertex_state(),
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                fragment: Some(FragmentState {
                    shader,
                    shader_defs: vec![],
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
            });

        Self {
            id,
            layout,
            sampler,
        }
    }
}
