use crate::prelude::*;
use sickle_ui::prelude::*;

pub struct PlayerTurnPlugin;
impl Plugin for PlayerTurnPlugin {
    fn build(&self, app: &mut App) {
        app.observe(on_tile_clicked);
    }
}

#[derive(Component)]
pub struct ActionCard;

// spawn interaction panel
fn on_tile_clicked(
    trigger: Trigger<TileClickEvent>,
    mut cmd: Commands,
    tiles: Query<(Entity, &GlobalTransform), With<Tile>>,
    open_panels: Query<Entity, With<ActionCard>>,
    window: Query<&Window>,
) {
    let Ok((_, global)) = tiles.get(trigger.entity()) else {
        return;
    };

    open_panels
        .iter()
        .for_each(|ent| cmd.entity(ent).despawn_recursive());

    let panel_anchor_position = window
        .get_single()
        .ok()
        .map(|win| win.cursor_position())
        .flatten()
        .unwrap_or_default()
        + Vec2::new(20., -50.);

    cmd.ui_builder(UiRoot)
        .div(|builder| {
            builder.action_panel(
                trigger.entity(),
                panel_anchor_position,
            );
        })
        .insert(StateScoped(ControlFlow::Playing))
        .insert(ActionCard);
}
