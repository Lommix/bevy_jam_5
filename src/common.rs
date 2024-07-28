use bevy::prelude::*;

#[allow(unused)]
pub mod prelude {
    pub use super::Free;
}

pub struct CommonPlugin;
impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, free);
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
