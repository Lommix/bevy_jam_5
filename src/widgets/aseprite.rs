use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use sickle_ui::prelude::*;

pub trait AsepriteExt {
    fn ase_image(
        &mut self,
        handle: Handle<Aseprite>,
        slice: &str,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;

    fn ase_animation(
        &mut self,
        handle: Handle<Aseprite>,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

impl AsepriteExt for UiBuilder<'_, Entity> {
    fn ase_image(
        &mut self,
        handle: Handle<Aseprite>,
        slice: &str,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        self.container(
            (
                NodeBundle::default(),
                AsepriteSliceUiBundle {
                    slice: AsepriteSlice::from(slice),
                    aseprite: handle,
                    ..default()
                },
            ),
            spawn_children,
        )
    }

    fn ase_animation(
        &mut self,
        handle: Handle<Aseprite>,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        self.container(
            (
                NodeBundle::default(),
                AsepriteAnimationUiBundle {
                    aseprite: handle.clone(),
                    ..default()
                },
            ),
            spawn_children,
        )
    }
}
