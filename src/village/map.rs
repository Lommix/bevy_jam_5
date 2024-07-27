use crate::prelude::*;
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    utils::HashMap,
    window::PrimaryWindow,
};
use bevy_aseprite_ultra::prelude::*;

#[allow(unused)]
pub mod prelude {
    pub use super::{Tile, TileClickEvent, VillageTileamap};
}

pub struct VillageTileamap;
impl Plugin for VillageTileamap {
    fn build(&self, app: &mut App) {
        app.add_event::<TileClickEvent>();
        app.add_systems(OnEnter(AppState::Playing), spawn_map);
        app.add_systems(Update, switch_season_animation);
        app.observe(highlight_tile);
        app.add_systems(
            Update,
            check_clicked.run_if(in_state(ControlFlow::PlayerTurn)),
        );
    }
}

fn highlight_tile(
    trigger: Trigger<TileClickEvent>,
    mut cmd: Commands,
    children: Query<&Children>,
    highlighted: Query<Entity, With<Highlight>>,
) {
    highlighted.iter().for_each(|ent| {
        cmd.entity(ent).remove::<Highlight>();
    });
    cmd.rec_insert::<Highlight>(trigger.entity(), &children);
}

#[derive(Event)]
pub struct TileClickEvent;

#[derive(Component)]
pub struct TileMap {
    pub map_size: IVec2,
    pub tile_size: Vec2,
    pub tiles: HashMap<IVec2, Entity>,
}

#[derive(Component)]
pub struct Tile;

#[derive(Bundle)]
pub struct TileBundle {
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub tile: Tile,
}

fn switch_season_animation(
    season: Res<State<Season>>,
    mut tiles: Query<&mut Animation, With<Tile>>,
) {
    tiles.iter_mut().for_each(|mut animation| {
        animation.tag = Some(season.animation().into());
    });
}

//@todo: touch control
fn check_touch(mut touch_events: EventReader<MouseButtonInput>) {}

fn check_clicked(
    mut cmd: Commands,
    mut events: EventReader<MouseButtonInput>,
    window: Query<&Window, With<PrimaryWindow>>,
    map: Query<&TileMap>,
    tiles: Query<(Entity, &GlobalTransform), With<Tile>>,
) {
    let Ok(tile_map) = map.get_single() else {
        return;
    };

    events.read().for_each(|event| {
        if matches!(event.state, ButtonState::Pressed) {
            return;
        };

        let Ok(window) = window.get(event.window) else {
            return;
        };

        let Some(click_position) =
            window.cursor_position().map(|pos| {
                (pos - window.size() / 2.) * Vec2::new(1., -1.)
            })
        else {
            return;
        };

        let Some((entity, position)) = tiles
            .iter()
            .map(|(ent, global)| {
                (ent, global.translation().truncate())
            })
            .filter(|(_, pos)| {
                collide_aabb(
                    *pos,
                    tile_map.tile_size / 2.,
                    click_position,
                )
            })
            .next()
        else {
            return;
        };

        cmd.trigger_targets(TileClickEvent, entity);
    });
}

fn collide_aabb(center: Vec2, half: Vec2, probe: Vec2) -> bool {
    let inside_x = probe.x >= (center.x - half.x)
        && probe.x <= (center.x + half.x);
    let inside_y = probe.y >= (center.y - half.y)
        && probe.y <= (center.y + half.y);
    inside_x && inside_y
}

fn spawn_map(mut cmd: Commands, sprites: Res<SpriteAssets>) {
    let gap = Vec2::new(1., 1.);
    let tile_size = Vec2::new(64., 64.);
    let mut tiles = HashMap::new();
    for x in -2..=2 {
        for y in -2..=2 {
            let offset = IVec2::new(x, y);
            let transform = Transform::from_translation(
                (offset.as_vec2() * tile_size
                    + gap * offset.as_vec2())
                .extend(0.),
            );
            let tile = cmd
                .spawn((
                    AsepriteAnimationBundle {
                        transform,
                        aseprite: sprites.tile.clone(),
                        ..default()
                    },
                    Tile,
                ))
                .id();

            if offset == IVec2::ZERO {
                cmd.entity(tile).insert(Building).with_children(
                    |cmd| {
                        cmd.spawn(AsepriteAnimationBundle {
                            aseprite: sprites.house.clone(),
                            transform: Transform::from_translation(
                                Vec3::Z,
                            ),
                            ..default()
                        });
                    },
                );
            }

            tiles.insert(offset, tile);
        }
    }

    let entities = tiles.iter().map(|(_, v)| *v).collect::<Vec<_>>();
    cmd.spawn((
        Name::new(""),
        SpatialBundle::default(),
        TileMap {
            map_size: IVec2::new(5, 5),
            tile_size,
            tiles,
        },
        StateScoped(AppState::Playing),
        Button,
        Interaction::None,
    ))
    .push_children(entities.as_slice());
}
