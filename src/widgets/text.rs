use crate::prelude::*;
use sickle_ui::prelude::*;

pub struct TextPlugin;
impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_text_added);
    }
}

fn on_text_added(
    mut texts: Query<&mut Text, Added<Text>>,
    server: Res<AssetServer>,
) {
    texts.iter_mut().for_each(|mut text| {
        for section in text.sections.iter_mut() {
            section.style.font = server.load("big_blue.TTF");
        }
    });
}

#[derive(Component)]
pub struct LoadDefaultFont;

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
                    color: COLOR_FONT,
                    ..default()
                },
            ),
            |builder| {},
        )
    }
}
