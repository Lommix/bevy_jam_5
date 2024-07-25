use crate::prelude::*;
use sickle_ui::prelude::*;

pub struct PanelWidgetPlugin;
impl Plugin for PanelWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ComponentThemePlugin::<Panel>::default());
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

impl Panel {
    fn theme() -> Theme<Panel> {
        let base_theme =
            PseudoTheme::deferred(None, Panel::primary_style);
        Theme::new(vec![base_theme])
    }
    pub fn primary_style(
        style_builder: &mut StyleBuilder,
        theme: &ThemeData,
    ) {
        style_builder
            .border_color(Color::BLACK)
            .border(UiRect::all(Val::Px(5.)))
            .border_radius(BorderRadius::all(Val::Px(10.)))
            .padding(UiRect::all(Val::Px(20.)));
    }
}

impl DefaultTheme for Panel {
    fn default_theme() -> Option<Theme<Self>> {
        Panel::theme().into()
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
        let mut out = self.container(NodeBundle::default(), |panel_div| {

            panel_div
                .insert(Name::new("Panel"))
                .style()
                .position_type(PositionType::Relative);

            let panel_id = panel_div.id();


            // title
            if let Some(title) = config.title {
                panel_div.div_centered(|builder| {
                    panel.title = builder
                        .text(title.as_str(), Size::Medium)
                        .style()
                        .font_color(Color::WHITE)
                        .id();
                    builder.style()
                        .padding(UiRect::all(Val::Px(2.)))
                        .justify_content(
                            JustifyContent::Center,
                        )
                        .width(Val::Percent(50.))
                        .top(Val::Px(-16.))
                        .position_type(PositionType::Absolute)
                        .align_self(AlignSelf::Center)
                        .height(Val::Px(30.))
                        .border_color(Color::BLACK)
                        .border_radius(BorderRadius::all(
                            Val::Px(5.),
                        ))
                        .border(UiRect::all(Val::Px(2.)))
                        .background_color(Color::BLACK);
                });
            }

            // close
            if config.close_button {
                panel.close = panel_div.container(ButtonBundle::default(),|builder| {

                    builder.style()
                        .position_type(PositionType::Absolute)
                        .bottom(Val::Px(-15.))
                        .align_self(AlignSelf::Center)
                        .padding(UiRect::all(Val::Px(2.)))
                        .border(UiRect::all(Val::Px(2.)))
                        .border_color(Color::BLACK)
                        .background_color(Color::BLACK)
                        .border_radius(BorderRadius::all(Val::Px(5.)));

                    builder
                        .text("close", Size::Small)
                        .style()
                        .font_color(Color::WHITE);

                    builder.entity_commands()
                        .observe( move |_trigger: Trigger<ButtonClicked>,
                            mut cmd: Commands| {
                                cmd.trigger_targets(PanelClosed, panel_id);
                                cmd.entity(panel_id).despawn_recursive();
                            });
                }).id();
            }

            spawn_children(panel_div)
        });

        out.insert(panel);
        out
    }
}
