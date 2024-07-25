use crate::prelude::*;
use sickle_ui::prelude::*;

pub struct DialogWidgetPlugin;
impl Plugin for DialogWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DialogClosed>();
    }
}

#[derive(Component)]
pub struct DialogWidget;

#[derive(Event)]
pub struct DialogClosed;

pub trait DialogExt {
    fn dialog(&mut self, text: &str) -> UiBuilder<Entity>;
}

impl DialogExt for UiBuilder<'_, Entity> {
    fn dialog(&mut self, text: &str) -> UiBuilder<Entity> {
        let mut dialog_id = Entity::PLACEHOLDER;

        self.panel_bg(PanelConfig::default().with_close(), |panel| {
            panel
                .style()
                .min_width(Val::Px(400.))
                .min_height(Val::Px(100.))
                .background_color(COLOR_PRIMARY)
                .border_color(COLOR_TER);

            panel.entity_commands().observe(
                move |trigger: Trigger<PanelClosed>,
                      mut cmd: Commands| {
                    cmd.trigger_targets(
                        DialogClosed,
                        trigger.entity(),
                    )
                },
            );

            panel
                .text(text, Size::Small)
                .style()
                .font_color(COLOR_FONT);
        })
    }
}
