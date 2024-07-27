use crate::prelude::*;
use sickle_ui::prelude::*;

pub struct DialogWidgetPlugin;
impl Plugin for DialogWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DialogClosed>();
    }
}

#[derive(Component)]
pub struct DialogWidget {
    text: Entity,
}

impl Default for DialogWidget {
    fn default() -> Self {
        Self {
            text: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Event)]
pub struct DialogClosed;

pub trait DialogExt {
    fn dialog(&mut self, text: &str) -> UiBuilder<Entity>;
}

impl DialogExt for UiBuilder<'_, Entity> {
    fn dialog(&mut self, text: &str) -> UiBuilder<Entity> {
        let mut widget = DialogWidget::default();
        let mut out = self.panel_bg(
            PanelConfig::default().with_close(),
            |panel| {
                panel
                    .style()
                    .min_width(Val::Px(400.))
                    .min_height(Val::Px(100.));

                panel.entity_commands().observe(
                    move |trigger: Trigger<PanelClosed>,
                          mut cmd: Commands| {
                        cmd.trigger_targets(
                            DialogClosed,
                            trigger.entity(),
                        )
                    },
                );

                widget.text = panel
                    .text(text, Size::Small)
                    .style()
                    .font_color(COLOR_FONT)
                    .id();
            },
        );

        out.insert(widget);
        out
    }
}
