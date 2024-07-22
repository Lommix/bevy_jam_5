use super::Size;
use bevy::prelude::*;
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
    buttons: Query<(Entity, &Interaction), With<ButtonWidget>>,
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

#[derive(Reflect, Component, Debug)]
#[reflect]
pub struct ButtonWidget {
    label: Entity,
    size: Size,
}

impl Default for ButtonWidget {
    fn default() -> Self {
        Self {
            label: Entity::PLACEHOLDER,
            size: Size::default(),
        }
    }
}

impl UiContext for ButtonWidget {
    fn get(&self, target: &str) -> Result<Entity, String> {
        match target {
            ButtonWidget::TEXT => Ok(self.label),
            _ => Err(format!(
                "{} doesn't exists for button witdget. Possible contexts: {:?}",
                target,
                self.contexts()
            )),
        }
    }

    fn cleared_contexts(&self) -> Vec<&'static str> {
        self.contexts()
    }

    fn contexts(&self) -> Vec<&'static str> {
        vec![ButtonWidget::TEXT]
    }
}

impl ButtonWidget {
    pub const TEXT: &'static str = "TEXT";

    pub fn primary_style(
        style_builder: &mut StyleBuilder,
        theme_data: &ThemeData,
    ) {
        style_builder
            .display(Display::Flex)
            .justify_content(JustifyContent::Center)
            .align_items(AlignItems::Center)
            .padding(UiRect::axes(Val::Px(5.), Val::Px(10.)))
            .border_radius(BorderRadius::all(Val::Px(10.)))
            .border(UiRect::all(Val::Px(2.)))
            .border_color(theme_data.colors().primary)
            .interactive()
            .background_color(InteractiveVals {
                idle: theme_data.colors().surface_dim,
                hover: theme_data.colors().surface_bright.into(),
                ..default()
            });

        style_builder
            .switch_target(ButtonWidget::TEXT)
            .font_color(LinearRgba::GREEN)
            .animated()
            .font_size(AnimatedVals {
                idle: 20.,
                hover: Some(40.),
                ..default()
            })
            .copy_from(theme_data.interaction_animation);

        style_builder.interactive().font_color(InteractiveVals {
            idle: theme_data.colors().on_surface,
            hover: theme_data.colors().on_surface_variant.into(),
            ..default()
        });
    }

    fn frame() -> impl Bundle {
        (Name::new("Button"), ButtonBundle::default())
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
        text: &str,
        size: Size,
        tags: impl Bundle,
    ) -> UiBuilder<Entity>;
}

impl ButtonWidgetExt for UiBuilder<'_, Entity> {
    fn button(
        &mut self,
        text: &str,
        size: Size,
        tags: impl Bundle,
    ) -> UiBuilder<Entity> {
        let mut button = ButtonWidget::default();
        button.size = size;

        let mut input = self.container(
            (ButtonWidget::frame(), tags),
            |builder| {
                button.label = builder
                    .spawn(TextBundle::from_section(
                        text,
                        TextStyle::default(),
                    ))
                    .id();
            },
        );
        input.insert(button);
        input
    }
}
