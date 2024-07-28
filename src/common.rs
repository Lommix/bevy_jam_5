use bevy::prelude::*;

#[allow(unused)]
pub mod prelude {
    pub use super::{Cooldown, Free, Lifetime};
}

pub struct CommonPlugin;
impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, (free, tick_cooldown, tick_lifetime));
    }
}

#[derive(Component, DerefMut, Deref, Debug)]
pub struct Target(Entity);

#[derive(Component, Default)]
pub struct Free;

fn free(mut cmd: Commands, query: Query<Entity, With<Free>>) {
    query.iter().for_each(|entity| {
        _ = cmd.get_entity(entity).map(|cmd| cmd.despawn_recursive());
    });
}

#[derive(Component, Deref, DerefMut)]
pub struct Lifetime(f32);

impl Lifetime {
    pub fn seconds(sec: f32) -> Self {
        Self(sec)
    }
}

fn tick_lifetime(
    mut cmd: Commands,
    mut lifetimes: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    lifetimes.iter_mut().for_each(|(entity, mut life)| {
        **life -= time.delta_seconds();
        if **life <= 0. {
            cmd.entity(entity).despawn_recursive();
        }
    });
}

#[derive(Component, Deref, DerefMut)]
pub struct Cooldown(f32);
impl Cooldown {
    pub fn seconds(sec: f32) -> Self {
        Self(sec)
    }
}
fn tick_cooldown(
    mut cmd: Commands,
    mut cooldowns: Query<(Entity, &mut Cooldown)>,
    time: Res<Time>,
) {
    cooldowns.iter_mut().for_each(|(entity, mut cd)| {
        **cd -= time.delta_seconds();
        if **cd <= 0. {
            cmd.entity(entity).remove::<Cooldown>();
        }
    });
}
