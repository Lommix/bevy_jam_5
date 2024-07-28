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
                            "You did not met the expected production quota. The local Lord had you executed! {} Years survived",
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
    trigger: Trigger<ButtonClicked>,
    mut state: ResMut<NextState<AppState>>,
    mut flow: ResMut<NextState<ControlFlow>>,
) {
    state.set(AppState::Menu);
    flow.set(ControlFlow::Intro);
}
