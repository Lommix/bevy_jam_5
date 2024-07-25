use crate::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use sickle_ui::prelude::*;

#[derive(Component)]
pub struct ButtonIconWidget {
    pub text: Entity,
    pub icon: Entity,
}

impl Default for ButtonIconWidget {
    fn default() -> Self {
        Self {
            text: Entity::PLACEHOLDER,
            icon: Entity::PLACEHOLDER,
        }
    }
}

pub trait ButtonIconExt {
    fn button_item(
        &mut self,
        handle: Handle<Aseprite>,
        slice: &str,
        text: &str,
    ) -> UiBuilder<Entity>;
}

impl ButtonIconExt for UiBuilder<'_, Entity> {
    fn button_item(
        &mut self,
        handle: Handle<Aseprite>,
        slice: &str,
        text: &str,
    ) -> UiBuilder<Entity> {
        let mut widget = ButtonIconWidget::default();
        let mut out = self.button(|builder| {
            builder
                .style()
                .column_gap(Val::Px(10.))
                .align_items(AlignItems::Center);

            widget.icon = builder
                .ase_image(handle.clone(), slice, |_| {})
                .style()
                .width(Val::Px(32.))
                .height(Val::Px(32.))
                .id();

            widget.text = builder
                .spawn(TextBundle::from_section(
                    text,
                    TextStyle::default(),
                ))
                .style()
                .font_size(20.)
                .font_color(Color::BLACK)
                .id();
        });
        out.insert(widget);
        out
    }
}
