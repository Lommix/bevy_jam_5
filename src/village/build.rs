use crate::{prelude::*, ron_asset_loader};
use bevy::render::view::RenderLayers;
use bevy_aseprite_ultra::prelude::*;
use serde::{Deserialize, Serialize};
use sickle_ui::prelude::*;

pub struct BuildPlugin;
impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<BuildingAsset>();
        app.add_plugins(BuildingAssetPlugin);
        app.add_systems(
            Update,
            (
                switch_season_animation,
                render_building,
                start_producing,
            )
                .in_set(GameSysSets::InGame),
        )
        .observe(spawn_build_menu);

        app.add_systems(
            OnExit(ControlFlow::Autoplay),
            progress_buildings,
        );
    }
}

ron_asset_loader!(
    BuildingAssetPlugin,
    BuildingAssetLoader,
    BuildingAsset,
    &["build.ron"],
    sprite -> sprite_handle
    =produces -> produce_handles
);

#[derive(Component)]
pub struct Building;

#[derive(Component, Deref, DerefMut)]
pub struct BuildProgress(u32);

#[derive(Bundle, Default)]
pub struct BuildingBundle {
    pub building: Handle<BuildingAsset>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

#[derive(Asset, TypePath, Serialize, Deserialize)]
pub struct BuildingAsset {
    pub name: String,
    pub description: String,
    pub sprite: String,
    pub produces: Vec<String>,

    pub build_cost: Option<f32>,
    pub build_time: u32,

    #[serde(skip)]
    pub sprite_handle: Handle<Aseprite>,

    #[serde(skip)]
    pub produce_handles: Vec<Handle<WorkOrder>>,
}

#[derive(Component)]
pub struct BuildOrderPanel;

fn spawn_build_menu(
    trigger: Trigger<TileClickEvent>,
    mut cmd: Commands,
    tiles: Query<
        (Entity, &GlobalTransform),
        (With<Tile>, Without<Children>),
    >,
    window: Query<&Window>,
    center_ui: Query<Entity, With<CenterRightUi>>,
    open_panels: Query<Entity, With<BuildOrderPanel>>,
    children: Query<&Children>,
) {
    let Ok(center) = center_ui.get_single() else {
        return;
    };

    let Ok((_, global)) = tiles.get(trigger.entity()) else {
        return;
    };

    open_panels
        .iter()
        .for_each(|ent| cmd.entity(ent).despawn_recursive());

    let pos = window
        .get_single()
        .ok()
        .map(|win| win.cursor_position())
        .flatten()
        .unwrap_or_default()
        + Vec2::new(20., -50.);

    cmd.ui_builder(UiRoot)
        .div(|builder| {
            builder.build_panel(trigger.entity(), pos);
        })
        .insert(BuildOrderPanel)
        .insert(StateScoped(ControlFlow::PlayerTurn));
}

fn switch_season_animation(
    season: Res<State<Season>>,
    mut tiles: Query<&mut Animation, With<Handle<BuildingAsset>>>,
) {
    tiles.iter_mut().for_each(|mut animation| {
        animation.tag = Some(season.animation().into());
    });
}

fn progress_buildings(
    mut cmd: Commands,
    mut build_sides: Query<
        (Entity, &mut BuildProgress, &Handle<BuildingAsset>),
        With<Tile>,
    >,
    assets: Res<Assets<BuildingAsset>>,
) {
    build_sides.iter_mut().for_each(
        |(entity, mut progress, handle)| {
            let Some(building) = assets.get(handle) else {
                return;
            };

            **progress += 1;

            if **progress >= building.build_time {
                cmd.entity(entity)
                    .remove::<Handle<Aseprite>>()
                    .remove::<BuildProgress>();
            }
        },
    );
}

fn render_building(
    mut cmd: Commands,
    query: Query<
        (Entity, &Handle<BuildingAsset>),
        Without<Handle<Aseprite>>,
    >,
    season: Res<State<Season>>,
    in_progress: Query<&BuildProgress>,
    buildings: Res<Assets<BuildingAsset>>,
    sprites: Res<SpriteAssets>,
) {
    query.iter().for_each(|(entity, handle)| {
        let Some(building) = buildings.get(handle) else {
            return;
        };

        match in_progress.get(entity) {
            Ok(_) => {
                cmd.entity(entity).insert(AsepriteAnimationBundle {
                    transform: Transform::from_translation(Vec3::Z),
                    aseprite: sprites.construct.clone(),
                    animation: Animation {
                        tag: Some(season.get().animation().into()),
                        ..default()
                    },
                    ..default()
                });
            }
            Err(_) => {
                cmd.entity(entity).insert(AsepriteAnimationBundle {
                    transform: Transform::from_translation(Vec3::Z),
                    aseprite: building.sprite_handle.clone(),
                    animation: Animation {
                        tag: Some(season.get().animation().into()),
                        ..default()
                    },
                    ..default()
                });
            }
        };
    });
}

fn start_producing(
    mut cmd: Commands,
    player: Query<Entity, (With<Inventory>, With<Player>)>,
    query: Query<
        (Entity, &Handle<BuildingAsset>),
        Without<Handle<WorkOrder>>,
    >,
    buildings: Res<Assets<BuildingAsset>>,
) {
    let Ok(player) = player.get_single() else {
        warn!("game over? still producing?");
        return;
    };

    query.iter().for_each(|(entity, handle)| {
        let Some(building) = buildings.get(handle) else {
            return;
        };

        let Some(handle) = building.produce_handles.first() else {
            return;
        };

        info!("starting work order");

        cmd.entity(entity).insert((
            WorkOrderBundle {
                order: handle.clone(),
                target: TargetInventory(player),
                working: Working::default(),
            },
            LoopOrder,
        ));
    });
}
