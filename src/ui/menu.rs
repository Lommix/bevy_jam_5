use super::{AppState, ControlFlow};
use crate::{assets::AudioAssets, widgets::prelude::*};
use bevy::{audio::Volume, prelude::*};
use sickle_ui::prelude::*;

pub struct GameMenuPlugin;
impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Menu),
            (spawn_menu, start_music),
        );
    }
}

#[derive(Component)]
pub struct MenuMusicTag;

pub(crate) fn start_music(
    mut cmd: Commands,
    query: Query<Entity, With<MenuMusicTag>>,
    audio_res: Res<AudioAssets>,
) {
    match query.get_single() {
        Ok(ent) => {
            cmd.entity(ent).insert(audio_res.menu_music.clone());
        }
        Err(_) => {
            cmd.spawn((
                AudioBundle {
                    source: audio_res.menu_music.clone(),
                    settings: PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Loop,
                        volume: Volume::new(0.3),
                        ..default()
                    },
                },
                MenuMusicTag,
            ));
        }
    };
}

pub(crate) fn spawn_menu(mut cmd: Commands) {
    cmd.ui_builder(UiRoot)
        .container(NodeBundle::default(), |root| {
            root.style()
                .width(Val::Percent(100.))
                .height(Val::Percent(100.))
                .display(Display::Flex)
                .justify_content(JustifyContent::Center)
                .align_items(AlignItems::Center);

            root.panel_bg(PanelConfig::title("Menu"), |container| {
                container
                    .style()
                    .flex_direction(FlexDirection::Column)
                    .row_gap(Val::Px(5.));

                container
                    .text("Brew Tycoon", Size::Large)
                    .style()
                    .padding(UiRect::axes(Val::Px(5.), Val::Px(35.)));

                container
                    .button(|builder| {
                        builder.text("Play Game", Size::Large);
                    })
                    .style()
                    .width(Val::Percent(100.))
                    .entity_commands()
                    .observe(start_game);
            });
        })
        .insert(StateScoped(AppState::Menu));
}

fn start_game(
    _trigger: Trigger<ButtonClicked>,
    mut next_state: ResMut<NextState<AppState>>,
    mut next_flow: ResMut<NextState<ControlFlow>>,
) {
    next_state.set(AppState::Playing);
    next_flow.set(ControlFlow::Intro);
}
