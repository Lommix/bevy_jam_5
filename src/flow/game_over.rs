use crate::prelude::*;
use sickle_ui::prelude::*;

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>();
        app.observe(spawn_score);
    }
}

#[derive(Event)]
pub enum GameOverEvent {
    Hunger,
    Debt,
}

#[derive(Component)]
struct ScoreTag;

fn spawn_score(
    trigger: Trigger<GameOverEvent>,
    mut cmd: Commands,
    mut state: ResMut<NextState<ControlFlow>>,
    village: Query<(&Village, &GameContext), With<Player>>,
    ui: Query<Entity, With<CenterMiddleUi>>,
) {
    let Ok(ui) = ui.get_single() else {
        return;
    };

    let Ok((village, context)) = village.get_single() else {
        return;
    };

    state.set(ControlFlow::Score);

    cmd.ui_builder(ui)
        .div_centered(|builder| {
            builder.insert(ScoreTag);
            builder
                .panel_bg(PanelConfig::title("Game Over"), |panel| {

                    let message = match trigger.event(){
                        GameOverEvent::Hunger => "Your villagers died of hunger. You lost.",
                        GameOverEvent::Debt => "Your debt was to high, the local lord had you executed.",
                    };

                    panel.text(
                        format!(
                            "{}. {} Years survived",
                            message,
                            context.current_year()
                        )
                        .as_str(),
                        Size::Medium,
                    );
                    panel
                        .button(|builder| {
                            builder
                                .text("Back to Menu", Size::Medium);
                        })
                        .style()
                        .margin(UiRect::top(Val::Px(50.)))
                        .padding(UiRect::axes(Val::Px(15.), Val::Px(10.)))
                        .entity_commands()
                        .observe(back_to_menu);
                })
                .style()
                .flex_direction(FlexDirection::Column)
                .z_index(ZIndex::Global(10));
        })
        .insert(StateScoped(ControlFlow::Score));
}

fn back_to_menu(
    _trigger: Trigger<ButtonClicked>,
    mut cmd: Commands,
    score: Query<Entity, With<ScoreTag>>,
    mut state: ResMut<NextState<AppState>>,
) {
    state.set(AppState::Menu);

    let Ok(s) = score.get_single() else {
        return;
    };

    cmd.entity(s).despawn_recursive();
}
