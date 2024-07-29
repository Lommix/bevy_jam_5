use crate::prelude::*;
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    utils::HashMap,
    window::PrimaryWindow,
};
use bevy_aseprite_ultra::prelude::*;

#[allow(unused)]
pub mod prelude {
    pub use super::{
        collide_aabb, Tile, TileClickEvent, VillageTileamap,
    };
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
            check_clicked.run_if(in_state(ControlFlow::Playing)),
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
fn check_touched(mut touch_events: EventReader<MouseButtonInput>) {}

fn check_clicked(
    mut cmd: Commands,
    inputs: Res<ButtonInput<MouseButton>>,
    cursor_state: Res<State<CursorState>>,
    crusor_query: Query<&GlobalTransform, With<Cursor>>,
    map: Query<&TileMap>,
    tiles: Query<(Entity, &GlobalTransform), With<Tile>>,
) {
    let Ok(tile_map) = map.get_single() else {
        return;
    };

    if !inputs.just_pressed(MouseButton::Left) {
        return;
    };

    if matches!(cursor_state.get(), CursorState::Ui) {
        return;
    }

    let Ok(global) = crusor_query.get_single() else {
        return;
    };

    let click_position = global.translation().truncate();

    let Some((entity, _)) = tiles
        .iter()
        .map(|(ent, global)| (ent, global.translation().truncate()))
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
}

pub fn collide_aabb(center: Vec2, half: Vec2, probe: Vec2) -> bool {
    let inside_x = probe.x >= (center.x - half.x)
        && probe.x <= (center.x + half.x);
    let inside_y = probe.y >= (center.y - half.y)
        && probe.y <= (center.y + half.y);
    inside_x && inside_y
}

fn spawn_map(
    mut cmd: Commands,
    sprites: Res<SpriteAssets>,
    buildings: Res<BuildingAssets>,
) {
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
                let building = cmd
                    .spawn(BuildingBundle {
                        building: buildings.house.clone(),
                        ..default()
                    })
                    .id();

                cmd.entity(tile).add_child(building);
            }

            tiles.insert(offset, tile);
        }
    }

    let entities = tiles.iter().map(|(_, v)| *v).collect::<Vec<_>>();
    cmd.spawn((
        Name::new("map"),
        SpatialBundle::default(),
        TileMap {
            map_size: IVec2::new(5, 5),
            tile_size,
            tiles,
        },
        StateScoped(AppState::Playing),
    ))
    .push_children(entities.as_slice());
}
