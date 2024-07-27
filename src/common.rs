use bevy::prelude::*;

#[derive(Component, DerefMut, Deref, Debug)]
pub struct Target(Entity);

