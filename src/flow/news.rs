use std::collections::VecDeque;

use crate::prelude::*;
use bevy::prelude::*;
use sickle_ui::prelude::*;

#[allow(unused)]
pub mod prelude {
    pub use super::NewsEvent;
}

#[rustfmt::skip]
// -----------------------------
// tutorial
pub const TUTORIAL: &[&'static str; 4] = &[
r#"
Welcome to your new village! Click on an empty tile to build a field and a lumberjack. Each Villager represent 1 workforce, consuming 1 food each season.
Each year you have to pay taxes. You make gold by crafting goods and sell them on the market. The local lords are gready, taxes increase each year.
"#,
r#"
Houses increase the villager cap by 4. Workstations can produce items from resources. These can be sold at the Market to make gold. Clicking on a house opens the village inventory
"#,
r#"
It's now your turn to build an trading empire.
"#,
r#"
Oh! And there are taxes! Try to build produce enough to please the local lord, if you value your head that is.
"#,
];

pub struct NewsPlugin;
impl Plugin for NewsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewsEvent>();
        app.add_systems(
            OnEnter(ControlFlow::Playing),
            spawn_new_dialog,
        );
        app.add_systems(
            Update,
            update_news.in_set(GameSysSets::InGame),
        );
    }
}

#[derive(Event)]
pub struct NewsEvent {
    pub message: String,
}

#[derive(Component)]
pub struct NewsDialog;

fn spawn_new_dialog(
    mut cmd: Commands,
    ui_query: Query<Entity, With<BottomUi>>,
) {
    let Ok(ui_ent) = ui_query.get_single() else {
        return;
    };

    cmd.ui_builder(ui_ent)
        .style()
        .justify_content(JustifyContent::Center);

    cmd.ui_builder(ui_ent)
        .dialog(TUTORIAL[0])
        .insert(StateScoped(ControlFlow::Playing))
        .insert(NewsDialog)
        .style()
        .position_type(PositionType::Absolute)
        .align_self(AlignSelf::End)
        .justify_self(JustifySelf::Center)
        .max_width(Val::Px(600.));
}

fn update_news(
    village: Query<(&Village, &GameContext), With<Player>>,
    mut events: EventReader<NewsEvent>,
    dialog: Query<&DialogWidget, With<NewsDialog>>,
    mut text: Query<&mut Text>,
    fonts: Res<FontAssets>,
) {
    let news =
        events.read().map(|e| e.message.clone()).collect::<Vec<_>>();

    if news.len() == 0 {
        return;
    }

    let Ok((village, context)) = village.get_single() else {
        return;
    };

    let Ok(dialog) = dialog.get_single() else {
        return;
    };

    let Ok(mut text) = text.get_mut(dialog.text) else {
        return;
    };

    text.sections = news
        .iter()
        .map(|m| TextSection {
            value: m.into(),
            style: TextStyle {
                font_size: 15.,
                color: COLOR_FONT,
                font: fonts.font.clone(),
                ..default()
            },
        })
        .collect::<Vec<_>>();
}

fn finish_news(
    _trigger: Trigger<DialogClosed>,
    mut next_state: ResMut<NextState<ControlFlow>>,
) {
    next_state.set(ControlFlow::Playing);
}
