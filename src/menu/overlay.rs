use crate::prelude::*;
use bevy::{audio::Volume, prelude::*};
use sickle_ui::prelude::*;

#[derive(States, Hash, Clone, PartialEq, Eq, Debug, Default)]
pub enum OverlayState {
    #[default]
    Hidden,
    Shown,
}

pub struct OverlayPlugin;
impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<OverlayState>()
            .add_systems(Update, adjust_volume)
            .add_systems(OnEnter(OverlayState::Shown), show_overlay);
    }
}

#[derive(Component, Default)]
struct VolumeSlider;

fn adjust_volume(
    mut volume: ResMut<GlobalVolume>,
    slider_query: Query<&Slider, With<VolumeSlider>>,
) {
    let Ok(slider) = slider_query.get_single() else {
        return;
    };

    volume.volume = Volume::new(slider.value());
}

fn show_overlay(mut cmd: Commands) {
    cmd.ui_builder(UiRoot).container(NodeBundle::default(), |builder|{
        builder
            .slider(SliderConfig{
                label: Some( "Volume".into() ),
                min: 0.,
                max: 1.,
                initial_value: 1.,
                show_current: true,
                axis: sickle_ui::widgets::inputs::slider::SliderAxis::Horizontal,
            })
            .insert(VolumeSlider)
            .style()
            .width(Val::Px(200.))
            .height(Val::Px(50.));
    }).insert(StateScoped(OverlayState::Shown));
}
