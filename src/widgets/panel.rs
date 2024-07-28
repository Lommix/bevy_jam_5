use crate::prelude::*;
use sickle_ui::prelude::*;

pub struct PanelWidgetPlugin;
impl Plugin for PanelWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PanelClosed>();
    }
}

const PANEL_PNG: &'static str = "ui/panel.png";

#[derive(Component, UiContext)]
pub struct Panel {
    pub title: Entity,
    pub close: Entity,
}

#[derive(Clone)]
pub struct PanelConfig {
    pub close_button: bool,
    pub title: Option<String>,
}

#[derive(Event)]
pub struct PanelClosed;

impl PanelConfig {
    pub fn title(title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            close_button: false,
        }
    }

    pub fn with_close(mut self) -> Self {
        self.close_button = true;
        self
    }
}

impl Default for PanelConfig {
    fn default() -> Self {
        Self {
            close_button: false,
            title: None,
        }
    }
}

impl Default for Panel {
    fn default() -> Self {
        Self {
            title: Entity::PLACEHOLDER,
            close: Entity::PLACEHOLDER,
        }
    }
}

pub trait PanelExt {
    fn panel_bg(
        &mut self,
        config: PanelConfig,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

impl PanelExt for UiBuilder<'_, Entity> {
    fn panel_bg(
        &mut self,
        config: PanelConfig,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        let mut panel = Panel::default();
        let mut out =
            self.container(NodeBundle::default(), |panel_div| {
                panel_div
                    .insert(Name::new("Panel"))
                    .style()
                    .position_type(PositionType::Relative)
                    .border_color(COLOR_ACCENT)
                    .background_color(COLOR_PRIMARY)
                    .border(UiRect::all(Val::Px(5.)))
                    .border_radius(BorderRadius::all(Val::Px(10.)))
                    .padding(UiRect::axes(
                        Val::Px(20.),
                        Val::Px(35.),
                    ));

                let panel_id = panel_div.id();

                // title
                if let Some(title) = config.title {
                    panel_div.div_centered(|builder| {
                        panel.title = builder
                            .text(title.as_str(), Size::Medium)
                            .style()
                            .font_color(COLOR_FONT)
                            .id();
                        builder
                            .style()
                            .padding(UiRect::all(Val::Px(2.)))
                            .justify_content(JustifyContent::Center)
                            .width(Val::Percent(50.))
                            .top(Val::Px(-16.))
                            .position_type(PositionType::Absolute)
                            .align_self(AlignSelf::Center)
                            .height(Val::Px(30.))
                            .border_color(COLOR_ACCENT)
                            .border_radius(BorderRadius::all(
                                Val::Px(5.),
                            ))
                            .border(UiRect::all(Val::Px(5.)))
                            .background_color(COLOR_ACCENT);
                    });
                }

                // close
                if config.close_button {
                    panel_div.div(|div| {
                        div.style()
                            .top(Val::Px(-16.))
                            .position_type(PositionType::Absolute)
                            .width(Val::Percent(100.))
                            .padding(UiRect::horizontal(Val::Px(20.)))
                            .justify_content(JustifyContent::End);

                        panel.close = div
                            .container(
                                ButtonBundle::default(),
                                |builder| {
                                    builder
                                        .style()
                                        .padding(UiRect::all(
                                            Val::Px(2.),
                                        ))
                                        .border(UiRect::all(Val::Px(
                                            4.,
                                        )))
                                        .border_color(COLOR_ACCENT)
                                        .background_color(
                                            COLOR_ACCENT,
                                        )
                                        .border_radius(
                                            BorderRadius::all(
                                                Val::Px(5.),
                                            ),
                                        );

                                    builder
                                        .text("X", Size::Medium)
                                        .style()
                                        .font_color(COLOR_FONT);

                                    builder
                                        .insert(CloseButton(panel_id))
                                        .entity_commands()
                                        .observe(on_panel_close);
                                },
                            )
                            .id();
                    });
                }

                spawn_children(panel_div)
            });

        out.insert(panel);
        out
    }
}

#[derive(Component, Deref)]
pub struct CloseButton(pub Entity);

fn on_panel_close(
    trigger: Trigger<ButtonClicked>,
    close_targets: Query<&CloseButton>,
    mut cmd: Commands,
) {
    let Ok(target) = close_targets.get(trigger.entity()) else {
        return;
    };

    cmd.trigger(ClearHighlights);
    cmd.trigger_targets(PanelClosed, **target);
    cmd.entity(**target).despawn_recursive();
}
