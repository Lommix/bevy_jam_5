use crate::prelude::*;
use sickle_ui::prelude::*;

pub struct ButtonWidgetPlugin;
impl Plugin for ButtonWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonClicked>()
            .add_plugins(
                ComponentThemePlugin::<ButtonWidget>::default(),
            )
            .add_systems(Update, button_events);
    }
}

#[derive(Event, Debug)]
pub struct ButtonClicked;

fn button_events(
    mut cmd: Commands,
    buttons: Query<(Entity, &Interaction)>,
) {
    buttons.iter().for_each(|(entity, interaction)| {
        match interaction {
            Interaction::Pressed => {
                cmd.trigger_targets(ButtonClicked, entity);
            }
            Interaction::None => {}
            Interaction::Hovered => {}
        };
    });
}

#[derive(Reflect, Component, UiContext, Debug, Default)]
#[reflect]
pub struct ButtonWidget;

impl ButtonWidget {
    pub fn primary_style(
        style_builder: &mut StyleBuilder,
        theme_data: &ThemeData,
    ) {
        style_builder
            .display(Display::Flex)
            .justify_content(JustifyContent::Center)
            .align_items(AlignItems::Center)
            .background_color(COLOR_SECONDARY)
            .border_color(COLOR_SECONDARY)
            .padding(UiRect::axes(Val::Px(2.), Val::Px(2.)))
            .border_radius(BorderRadius::all(Val::Px(5.)))
            .animated()
            .border_color(AnimatedVals {
                idle: COLOR_SECONDARY,
                hover: Some(COLOR_ACCENT),
                ..default()
            })
            .copy_from(theme_data.interaction_animation);
    }

    fn frame() -> impl Bundle {
        (Name::new("Button"), ButtonBundle::default(), ButtonWidget)
    }

    fn theme() -> Theme<ButtonWidget> {
        let base_theme =
            PseudoTheme::deferred(None, ButtonWidget::primary_style);
        Theme::new(vec![base_theme])
    }
}

impl DefaultTheme for ButtonWidget {
    fn default_theme() -> Option<Theme<Self>> {
        ButtonWidget::theme().into()
    }
}

pub trait ButtonWidgetExt {
    fn button(
        &mut self,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

impl ButtonWidgetExt for UiBuilder<'_, Entity> {
    fn button(
        &mut self,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        self.container(ButtonWidget::frame(), |builder| {
            spawn_children(builder);
        })
    }
}
