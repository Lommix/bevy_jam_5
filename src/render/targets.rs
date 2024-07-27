use bevy::{
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension,
            TextureFormat, TextureUsages,
        },
        texture::{BevyDefault, ImageSampler},
    },
    window::{PrimaryWindow, WindowResized},
};

pub struct RenderTargetPlugin;
impl Plugin for RenderTargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            ExtractResourcePlugin::<RenderTargets>::default(),
        )
        .add_systems(PreStartup, resize)
        .add_systems(
            First,
            resize.run_if(on_event::<WindowResized>()),
        );
    }
}

fn resize(
    mut cmd: Commands,
    mut images: ResMut<Assets<Image>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };

    info!("inserting targets!!");

    cmd.insert_resource(RenderTargets::from_size(
        window,
        &mut images,
    ));
}

#[derive(Resource, ExtractResource, Clone)]
pub struct RenderTargets {
    pub hight_light_source: Handle<Image>,
}

impl RenderTargets {
    pub fn from_size(
        window: &Window,
        images: &mut Assets<Image>,
    ) -> Self {
        let hight_light_source: Handle<Image> =
            Handle::weak_from_u128(605214787963254423236589025);

        images.insert(
            &hight_light_source,
            create_image(
                window.size(),
                TextureFormat::bevy_default(),
                ImageSampler::nearest(),
            ),
        );
        Self { hight_light_source }
    }
}

fn create_image(
    size: Vec2,
    format: TextureFormat,
    sampler: ImageSampler,
) -> Image {
    let size = Extent3d {
        width: size.x as u32,
        height: size.y as u32,
        depth_or_array_layers: 1,
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        sampler,
        ..default()
    };
    image.resize(size);
    image
}
