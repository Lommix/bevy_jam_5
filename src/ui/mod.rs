use crate::prelude::*;
use bevy::prelude::*;
use sickle_ui::{
    prelude::*,
    theme::theme_colors::{CoreColors, ThemeColors},
};

mod colors;
mod menu;
mod overlay;

#[allow(unused)]
pub mod prelude {
    pub use super::colors::*;
    pub use super::overlay::OverlayState;
    pub use super::{
        BottomUi, CenterLeftUi, CenterMiddleUi, CenterRightUi,
        MenuPlugin, TopUi,
    };
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            menu::GameMenuPlugin,
            overlay::OverlayPlugin,
        ))
        .add_systems(OnEnter(AppState::Startup), startup);

        app.insert_resource(ThemeData {
            colors: ThemeColors {
                core_colors: CoreColors {
                    primary: LinearRgba::BLACK.into(),
                    secondary: Some(LinearRgba::WHITE.into()),
                    tertiary: Some(LinearRgba::GREEN.into()),
                    error: Some(LinearRgba::RED.into()),
                    neutral: Some(LinearRgba::BLUE.into()),
                    neutral_variant: Some(
                        LinearRgba {
                            red: 0.3,
                            green: 0.3,
                            blue: 0.3,
                            alpha: 1.,
                        }
                        .into(),
                    ),
                },
                ..default()
            },
            ..default()
        });
    }
}

#[derive(Component)]
pub struct TopUi;
#[derive(Component)]
pub struct CenterLeftUi;
#[derive(Component)]
pub struct CenterMiddleUi;
#[derive(Component)]
pub struct CenterRightUi;
#[derive(Component)]
pub struct BottomUi;

fn startup(mut cmd: Commands, mut theme_data: ResMut<ThemeData>) {
    cmd.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec3::splat(0.80)),
        ..default()
    });

    theme_data.active_scheme = Scheme::Light(Contrast::Standard);

    cmd.ui_builder(UiRoot).container(
        NodeBundle::default(),
        |builder| {
            builder
                .style()
                .display(Display::Flex)
                .width(Val::Percent(100.))
                .height(Val::Percent(100.))
                .flex_direction(FlexDirection::Column)
                .padding(UiRect::all(Val::Px(10.)));

            builder
                .spawn((NodeBundle::default(), TopUi))
                .style()
                .width(Val::Percent(100.))
                .height(Val::Percent(100.));

            builder
                .container(NodeBundle::default(), |builder| {
                    builder
                        .style()
                        .flex_direction(FlexDirection::Row);
                    builder
                        .spawn((NodeBundle::default(), CenterLeftUi))
                        .style()
                        .width(Val::Percent(100.));
                    builder
                        .spawn((
                            NodeBundle::default(),
                            CenterMiddleUi,
                        ))
                        .style()
                        .width(Val::Percent(100.));
                    builder
                        .spawn((NodeBundle::default(), CenterRightUi))
                        .style()
                        .width(Val::Percent(100.));
                })
                .style()
                .width(Val::Percent(100.))
                .height(Val::Percent(100.));

            builder
                .spawn((NodeBundle::default(), BottomUi))
                .style()
                .width(Val::Percent(100.))
                .height(Val::Percent(100.));
        },
    );
}

fn end_game(
    _trigger: Trigger<ButtonClicked>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    info!("end game");
    next_state.set(AppState::Menu);
}
