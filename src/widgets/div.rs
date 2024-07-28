use bevy::prelude::*;
use sickle_ui::prelude::*;

pub trait DivExt {
    fn div(
        &mut self,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

pub trait DivCenteredExt {
    fn div_centered(
        &mut self,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

impl DivExt for UiBuilder<'_, UiRoot> {
    fn div(
        &mut self,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        self.container(NodeBundle::default(), |builder| {
            builder
                .style()
                .width(Val::Percent(100.))
                .height(Val::Percent(100.));

            spawn_children(builder);
        })
    }
}

impl DivExt for UiBuilder<'_, Entity> {
    fn div(
        &mut self,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        self.container(NodeBundle::default(), spawn_children)
    }
}

impl DivCenteredExt for UiBuilder<'_, Entity> {
    fn div_centered(
        &mut self,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        self.container(NodeBundle::default(), |builder| {
            builder
                .style()
                .width(Val::Percent(100.))
                .height(Val::Percent(100.))
                .display(Display::Flex)
                .justify_content(JustifyContent::Center)
                .align_items(AlignItems::Center);

            spawn_children(builder);
        })
    }
}

impl DivCenteredExt for UiBuilder<'_, UiRoot> {
    fn div_centered(
        &mut self,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        self.container(NodeBundle::default(), |builder| {
            builder
                .style()
                .width(Val::Percent(100.))
                .height(Val::Percent(100.))
                .display(Display::Flex)
                .justify_content(JustifyContent::Center)
                .align_items(AlignItems::Center);

            spawn_children(builder);
        })
    }
}
