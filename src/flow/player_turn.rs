use crate::prelude::*;
use sickle_ui::prelude::*;

pub struct PlayerTurnPlugin;
impl Plugin for PlayerTurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ControlFlow::PlayerTurn), start_turn);
        app.observe(on_tile_clicked);
    }
}

// enable clicking
// make descision
// end turn
fn start_turn(
    mut cmd: Commands,
    query: Query<Entity, With<BottomUi>>,
) {
    let Ok(bottom_ui) = query.get_single() else {
        return;
    };

    cmd.ui_builder(bottom_ui)
        .div_centered(|builder| {
            builder
                .button(|button| {
                    button.style().padding(UiRect::axes(
                        Val::Px(20.),
                        Val::Px(10.),
                    ));

                    button.text("End this turn", Size::Large);
                })
                .entity_commands()
                .observe(end_turn);
        })
        .insert(StateScoped(ControlFlow::PlayerTurn));
}

fn end_turn(
    _trigger: Trigger<ButtonClicked>,
    mut flow: ResMut<NextState<ControlFlow>>,
    mut cmd: Commands,
) {
    cmd.trigger(ClearHighlights);
    flow.set(ControlFlow::Autoplay);
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
        .insert(ActionCard)
        .insert(StateScoped(ControlFlow::PlayerTurn));
}
