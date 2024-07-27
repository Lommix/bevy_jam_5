use super::AppState;
use crate::{assets::AudioAssets, widgets::prelude::*};
use bevy::prelude::*;
use sickle_ui::prelude::*;

pub struct GameMenuPlugin;
impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Menu),
            (spawn_menu /* start_music */,),
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
                    ..default()
                },
                MenuMusicTag,
                StateScoped(AppState::Menu),
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
                    .button(|builder| {
                        builder.text("Play Game", Size::Large);
                    })
                    .style()
                    .width(Val::Percent(100.))
                    .entity_commands()
                    .observe(start_game);

                container
                    .button(|builder| {
                        builder.text("Credits", Size::Large);
                    })
                    .style()
                    .width(Val::Percent(100.));
            });
        })
        .insert(StateScoped(AppState::Menu));
}

fn start_game(
    _trigger: Trigger<ButtonClicked>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    next_state.set(AppState::Playing);
}
