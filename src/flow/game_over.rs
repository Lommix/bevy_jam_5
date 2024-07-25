use crate::prelude::*;
use sickle_ui::prelude::*;

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_game_over);
        app.add_systems(OnEnter(ControlFlow::Score), spawn_score);
    }
}

fn check_game_over(
    village: Query<&Village, With<Player>>,
    mut state: ResMut<NextState<ControlFlow>>,
) {
    let Ok(vil) = village.get_single() else {
        return;
    };

    if vil.villager_count <= 0 {
        state.set(ControlFlow::Score);
    }
}

fn spawn_score(
    mut cmd: Commands,
    village: Query<(&Village, &GameContext), With<Player>>,
    ui: Query<Entity, With<CenterMiddleUi>>,
) {
    let Ok(ui) = ui.get_single() else {
        return;
    };

    let Ok((village, context)) = village.get_single() else {
        return;
    };

    cmd.ui_builder(ui)
        .div_centered(|builder| {
            builder
                .panel_bg(PanelConfig::title("Game Over"), |panel| {
                    panel.text(
                        format!(
                            "You survived {} Years",
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
                        .entity_commands()
                        .observe(back_to_menu);
                })
                .style()
                .flex_direction(FlexDirection::Column)
                .z_index(ZIndex::Global(10))
                .background_color(Color::WHITE);
        })
        .insert(StateScoped(ControlFlow::Score));
}

fn back_to_menu(
    trigger: Trigger<ButtonClicked>,
    mut state: ResMut<NextState<AppState>>,
    mut flow: ResMut<NextState<ControlFlow>>,
) {
    state.set(AppState::Menu);
    flow.set(ControlFlow::Intro);
}
