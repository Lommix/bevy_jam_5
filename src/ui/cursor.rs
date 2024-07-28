use crate::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_aseprite_ultra::prelude::*;

pub mod prelude {
    pub use super::{Cursor, CursorState};
}

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), spawn_cursor)
            .init_state::<CursorState>()
            .add_systems(
                First,
                (update_cursor, update_cursor_state).chain(),
            );
    }
}

#[derive(States, Default, Debug, Clone, Hash, PartialEq, Eq)]
pub enum CursorState {
    #[default]
    Game,
    Ui,
}

fn update_cursor_state(
    mut next_state: ResMut<NextState<CursorState>>,
    window: Query<&Window, With<PrimaryWindow>>,
    panels: Query<(&Node, &GlobalTransform), With<Panel>>,
) {
    let Some(cursor_pos) = window
        .get_single()
        .ok()
        .map(|w| w.cursor_position())
        .flatten()
    else {
        return;
    };

    let hover_ui = !panels.iter().all(|(node, transform)| {
        !collide_aabb(
            transform.translation().truncate(),
            node.size() / 2.,
            cursor_pos,
        )
    });

    if hover_ui {
        next_state.set(CursorState::Ui)
    } else {
        next_state.set(CursorState::Game)
    }
}

#[derive(Component)]
pub struct Cursor;

fn update_cursor(
    mut query: Query<&mut Transform, With<Cursor>>,
    window: Query<&Window>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };

    let Ok(mut transform) = query.get_single_mut() else {
        return;
    };

    if let Some(cursor_pos) = window.cursor_position() {
        let offset = 0.8
            * (cursor_pos - window.size() / 2.)
            * Vec2::new(1., -1.);

        transform.translation = offset.extend(100.);
    };
}

fn spawn_cursor(mut cmd: Commands, sprites: Res<SpriteAssets>) {
    cmd.spawn((
        Name::new("Cursor"),
        AsepriteSliceBundle {
            slice: "vodka".into(),
            aseprite: sprites.icons.clone(),
            ..default()
        },
        Cursor,
    ));
}
