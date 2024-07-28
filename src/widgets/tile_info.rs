use crate::prelude::*;
use sickle_ui::prelude::*;

#[allow(unused)]
pub mod prelude {
    pub use super::{TileInfoExt, TileInfoWidget};
}

pub struct TileInfoPanelPlugin;
impl Plugin for TileInfoPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_valid_actions);
    }
}

#[derive(Component)]
pub struct TileInfoWidget {
    pub tile: Entity,
    pub select: Entity,
}
impl Default for TileInfoWidget {
    fn default() -> Self {
        Self {
            tile: Entity::PLACEHOLDER,
            select: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Component, PartialEq, Eq)]
pub enum TileAction {
    Sell, //@todo: impl
    SetOrder(Handle<WorkOrder>),
}

#[derive(Component)]
struct ActionTarget {
    target: Entity,
    widget: Entity,
}

#[derive(Component)]
struct LoadActions;

fn load_valid_actions(
    mut cmd: Commands,
    info_panels: Query<(Entity, &TileInfoWidget), With<LoadActions>>,
    children: Query<&Children>,
    work: Query<&Handle<WorkOrder>>,
    buildings: Query<&Handle<BuildingAsset>>,

    order_assets: Res<Assets<WorkOrder>>,
    building_assets: Res<Assets<BuildingAsset>>,
) {
    info_panels.iter().for_each(|(entity, widget)| {
        //clear any children
        cmd.entity(widget.select).clear_children();
        cmd.entity(entity).remove::<LoadActions>();

        // find first building
        let Some((building_ent, building)) = children
            .get(widget.tile)
            .ok()
            .map(|children| {
                children
                    .iter()
                    .flat_map(|child| {
                        buildings
                            .get(*child)
                            .ok()
                            .map(|build| (*child, build))
                    })
                    .next()
            })
            .flatten()
            .map(|(ent, handle)| {
                building_assets.get(handle).map(|b| (ent, b))
            })
            .flatten()
        else {
            return;
        };

        building.produce_handles.iter().for_each(|handle| {
            let Some(order) = order_assets.get(handle) else {
                return;
            };

            cmd.ui_builder(widget.select).button(|button| {
                button.style().padding(UiRect::axes(
                    Val::Px(15.),
                    Val::Px(10.),
                ));

                button.text(&order.name, Size::Small);

                let cur_work = work.get(building_ent);
                dbg!(&cur_work);

                // if active
                if work
                    .get(building_ent)
                    .map(|h| h.id() == handle.id())
                    .unwrap_or_default()
                {
                    button.style().background_color(COLOR_ACTIVE);
                }

                button
                    .insert((
                        ActionTarget {
                            target: building_ent,
                            widget: entity,
                        },
                        TileAction::SetOrder(handle.clone()),
                    ))
                    .entity_commands()
                    .observe(on_action_pressed);
            });
        });

        // -----------------------------------
        // delete button
        cmd.ui_builder(widget.select).button(|button| {
            button.text("Delete", Size::Small);
            button
                .style()
                .margin(UiRect::top(Val::Px(20.)))
                .background_color(COLOR_ERROR);

            button
                .insert((
                    ActionTarget {
                        target: building_ent,
                        widget: entity,
                    },
                    TileAction::Sell,
                ))
                .entity_commands()
                .observe(on_action_pressed);
        });
    });
}

fn on_action_pressed(
    trigger: Trigger<ButtonClicked>,
    mut cmd: Commands,
    widgets: Query<&TileInfoWidget>,
    actions: Query<(&TileAction, &ActionTarget)>,
    children: Query<&Children>,
) {
    // change order or delete building
    let Ok((action, target)) = actions.get(trigger.entity()) else {
        return;
    };

    match action {
        TileAction::Sell => {
            cmd.entity(target.target).insert(Free);
        }
        TileAction::SetOrder(handle) => {
            cmd.entity(target.target).insert(handle.clone());
        }
    }

    let Ok(widget) = widgets.get(target.widget) else {
        return;
    };

    _ = children.get(widget.select).map(|children| {
        for child in children.iter() {
            if *child == trigger.entity() {
                cmd.style(*child).background_color(COLOR_ACTIVE);
            } else {
                cmd.style(*child).background_color(COLOR_SECONDARY);
            }
        }
    });
}

pub trait TileInfoExt {
    fn tile_info(&mut self, tile: Entity) -> UiBuilder<Entity>;
}

impl TileInfoExt for UiBuilder<'_, Entity> {
    fn tile_info(&mut self, tile: Entity) -> UiBuilder<Entity> {
        let mut widget = TileInfoWidget::default();
        widget.tile = tile;

        self.panel_bg(
            PanelConfig::title("Produce").with_close(),
            |panel| {
                widget.select = panel
                    .div(|div| {
                        div.style()
                            .flex_direction(FlexDirection::Column)
                            .row_gap(Val::Px(5.));
                    })
                    .id();
                panel.insert((widget, LoadActions));
            },
        )
    }
}
