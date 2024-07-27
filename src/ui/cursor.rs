use crate::prelude::*;
use bevy_aseprite_ultra::prelude::*;

pub mod prelude {
    pub use super::Cursor;
}

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), spawn_cursor)
            .add_systems(Update, update_cursor);
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

        let offset =
            0.8 *
            (cursor_pos - window.size() / 2.) * Vec2::new(1., -1.);

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
