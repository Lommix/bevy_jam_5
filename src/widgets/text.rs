use crate::prelude::*;
use sickle_ui::prelude::*;

pub trait TextExt {
    fn text(&mut self, txt: &str, size: Size) -> UiBuilder<Entity>;
}

impl TextExt for UiBuilder<'_, Entity> {
    fn text(&mut self, txt: &str, size: Size) -> UiBuilder<Entity> {
        self.container(
            TextBundle::from_section(
                txt,
                TextStyle {
                    font_size: size.val(),
                    color: Color::BLACK,
                    ..default()
                },
            ),
            |builder| {},
        )
    }
}
