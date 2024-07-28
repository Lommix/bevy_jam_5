use crate::prelude::*;
use sickle_ui::prelude::*;

pub struct BuildPanelPlugin;
impl Plugin for BuildPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BuildPanelClose>();
        app.add_systems(
            Update,
            update_options.in_set(GameSysSets::InGame),
        );
    }
}

fn update_options(
    mut cmd: Commands,
    wigets: Query<
        (Entity, &BuildPanel),
        (With<BuildPanel>, Added<BuildPanel>),
    >,
    village: Query<
        (&Village, &Inventory, &GameContext),
        With<Player>,
    >,
    building_assets: Res<Assets<BuildingAsset>>,
    buildings: Res<BuildingAssets>,
) {
    let Ok((village, inventory, context)) = village.get_single()
    else {
        return;
    };

    for (panel_ent, panel) in wigets.iter() {
        // spawn button
        buildings.iter().for_each(|handle| {
            let Some(building) = building_assets.get(&handle) else {
                return;
            };

            if cmd.get_entity(panel_ent).is_none() {
                return;
            };

            // can build? -> todo
            cmd.ui_builder(panel_ent)
                .button(|button| {
                    button.text(&building.name, Size::Medium);
                })
                .insert(BuildOrder {
                    panel: panel_ent,
                    tile: panel.tile,
                    building: handle.clone(),
                })
                .entity_commands()
                .observe(build_order);
        })
    }
}

#[derive(Component)]
pub struct BuildOrder {
    panel: Entity,
    tile: Entity,
    building: Handle<BuildingAsset>,
}

#[derive(Component)]
pub struct HasOptions;

#[derive(Event)]
pub struct BuildPanelClose;

#[derive(Component)]
pub struct BuildPanel {
    tile: Entity,
}

impl Default for BuildPanel {
    fn default() -> Self {
        Self {
            tile: Entity::PLACEHOLDER,
        }
    }
}

pub trait BuildPanelExt {
    fn build_panel(&mut self, tile: Entity) -> UiBuilder<Entity>;
}

impl BuildPanelExt for UiBuilder<'_, Entity> {
    fn build_panel(&mut self, tile: Entity) -> UiBuilder<Entity> {
        let mut widget = BuildPanel::default();
        widget.tile = tile;

        let mut out = self.panel_bg(
            PanelConfig::title("Build").with_close(),
            |panel| {
                panel
                    .style()
                    .flex_direction(FlexDirection::Column)
                    .row_gap(Val::Px(5.));
            },
        );
        out.insert(widget);
        out
    }
}

fn build_order(
    trigger: Trigger<ButtonClicked>,
    mut cmd: Commands,
    build_orders: Query<&BuildOrder>,
) {
    let Ok(order) = build_orders.get(trigger.entity()) else {
        return;
    };

    let building = cmd
        .spawn(BuildingBundle {
            building: order.building.clone(),
            ..default()
        })
        .id();

    cmd.entity(order.tile).add_child(building);
    cmd.trigger_targets(PanelClosed, order.panel);
    cmd.entity(order.panel).despawn_recursive();
}
