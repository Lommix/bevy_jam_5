use super::GameState;
use crate::{assets::AudioAssets, ui::prelude::*};
use bevy::prelude::*;
use sickle_ui::prelude::*;

pub struct GameMenuPlugin;
impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Menu),
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
                    ..default()
                },
                MenuMusicTag,
                StateScoped(GameState::Menu),
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

            root.column(|container| {
                container
                    .style()
                    .width(Val::Px(200.))
                    .display(Display::Flex)
                    .justify_content(JustifyContent::Center)
                    .align_items(AlignItems::Center);

                container
                    .button("Play", Size::Medium, ())
                    .style()
                    .width(Val::Percent(100.))
                    .entity_commands()
                    .observe(start_game);

                container
                    .button("Settings", Size::Medium, ())
                    .style()
                    .width(Val::Percent(100.))
                    .entity_commands()
                    .observe(|_trigger: Trigger<ButtonClicked>| {
                        info!("settings?");
                    });

                container
                    .button("Credits", Size::Medium, ())
                    .style()
                    .width(Val::Percent(100.));
            });
        })
        .insert(StateScoped(GameState::Menu));
}

fn start_game(
    _trigger: Trigger<ButtonClicked>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Playing);
}
