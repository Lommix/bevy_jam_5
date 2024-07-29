use crate::prelude::*;
use sickle_ui::prelude::*;

pub struct VillageHudPlugin;
impl Plugin for VillageHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_hud.in_set(GameSysSets::InGame),
        );
    }
}

fn update_hud(
    villages: Query<(&Village, &Inventory)>,
    bag: Query<&Children>,
    huds: Query<&VillageHudWidget>,
    items: Query<(&Quantity, &Handle<ItemAsset>)>,
    eatable: Query<(), With<Eatable>>,
    seasons: Res<State<Season>>,
    mut texts: Query<&mut Text>,
    font: Res<FontAssets>,
    item_assets: Res<ItemAssets>,
) {
    huds.iter().for_each(|hud| {
        let Ok((village, inventory)) = villages.get(hud.village)
        else {
            return;
        };

        let Ok(item_enteties) = bag.get(inventory.bag) else {
            return;
        };

        let food_count = item_enteties
            .iter()
            .flat_map(|ent| {
                if eatable.get(*ent).is_err() {
                    return None;
                }

                items.get(*ent).ok()
            })
            .map(|(quant, _)| **quant)
            .sum::<i32>();

        let wood_count = item_enteties
            .iter()
            .flat_map(|ent| items.get(*ent).ok())
            .filter(|(_, handle)| {
                handle.id() == item_assets.wood.id()
            })
            .map(|(quant, _)| **quant)
            .sum::<i32>();

        _ = texts.get_mut(hud.season_txt).map(|mut txt| {
            let s = seasons.get().to_string();
            txt.sections = vec![s.into()];
        });

        _ = texts.get_mut(hud.wood_txt).map(|mut txt| {
            txt.sections = vec![TextSection::new(
                format!("{}", wood_count),
                TextStyle {
                    font: font.font.clone(),
                    ..default()
                },
            )];
        });
        _ = texts.get_mut(hud.food_txt).map(|mut txt| {
            txt.sections = vec![TextSection::new(
                format!("{}", food_count),
                TextStyle {
                    font: font.font.clone(),
                    ..default()
                },
            )];
        });
        _ = texts.get_mut(hud.gold_txt).map(|mut txt| {
            txt.sections = vec![TextSection::new(
                format!("{:.1}", inventory.gold),
                TextStyle {
                    font: font.font.clone(),
                    ..default()
                },
            )];
        });
        _ = texts.get_mut(hud.villager_count_txt).map(|mut txt| {
            let s = format!(
                "{}/{}",
                village.villager_count - village.villager_busy,
                village.villager_count
            );

            txt.sections = vec![TextSection::new(
                s,
                TextStyle {
                    font: font.font.clone(),
                    ..default()
                },
            )];
        });
    });
}

#[derive(Component, UiContext)]
pub struct VillageHudWidget {
    village: Entity,
    food_txt: Entity,
    wood_txt: Entity,
    gold_txt: Entity,
    villager_count_txt: Entity,
    season_txt: Entity,
}

impl Default for VillageHudWidget {
    fn default() -> Self {
        Self {
            village: Entity::PLACEHOLDER,
            food_txt: Entity::PLACEHOLDER,
            gold_txt: Entity::PLACEHOLDER,
            wood_txt: Entity::PLACEHOLDER,
            villager_count_txt: Entity::PLACEHOLDER,
            season_txt: Entity::PLACEHOLDER,
        }
    }
}

impl VillageHudWidget {}

pub trait VillageHudExt {
    fn village_hud(
        &mut self,
        village: Entity,
        sprites: &SpriteAssets,
    ) -> UiBuilder<Entity>;
}

impl VillageHudExt for UiBuilder<'_, Entity> {
    fn village_hud(
        &mut self,
        village: Entity,
        sprites: &SpriteAssets,
    ) -> UiBuilder<Entity> {
        let mut widget = VillageHudWidget::default();
        widget.village = village;
        let mut out = self.div_centered(|div| {
            div.style().width(Val::Px(600.)).height(Val::Px(40.));

            div.div(|builder| {
                builder
                    .style()
                    .column_gap(Val::Px(10.))
                    .justify_content(JustifyContent::SpaceBetween)
                    .flex_direction(FlexDirection::Row);

                builder
                    .button(|builder| {
                        builder
                            .ase_image(
                                sprites.icons.clone(),
                                "carrot",
                                |_| {},
                            )
                            .style()
                            .width(Val::Px(32.))
                            .height(Val::Px(32.));
                        widget.food_txt =
                            builder.text("food", Size::Medium).id();
                    })
                    .style()
                    .width(Val::Percent(100.));

                builder
                    .button(|builder| {
                        builder
                            .ase_image(
                                sprites.icons.clone(),
                                "wood",
                                |_| {},
                            )
                            .style()
                            .width(Val::Px(32.))
                            .height(Val::Px(32.));
                        widget.wood_txt =
                            builder.text("wood", Size::Medium).id();
                    })
                    .style()
                    .width(Val::Percent(100.));

                builder
                    .button(|builder| {
                        builder
                            .ase_image(
                                sprites.icons.clone(),
                                "gold",
                                |_| {},
                            )
                            .style()
                            .width(Val::Px(32.))
                            .height(Val::Px(32.));
                        widget.gold_txt =
                            builder.text("gold", Size::Medium).id();
                    })
                    .style()
                    .width(Val::Percent(100.));

                builder
                    .button(|builder| {
                        builder
                            .ase_image(
                                sprites.icons.clone(),
                                "villager",
                                |_| {},
                            )
                            .style()
                            .width(Val::Px(32.))
                            .height(Val::Px(32.));
                        widget.villager_count_txt = builder
                            .text("villager", Size::Medium)
                            .id();
                    })
                    .style()
                    .width(Val::Percent(100.));
            })
            .style()
            .width(Val::Percent(100.));

            div.div_centered(|builder| {
                builder
                    .button(|builder| {
                        widget.season_txt =
                            builder.text("season", Size::Small).id();
                    })
                    .style()
                    .width(Val::Px(100.));
            })
            .style()
            .position_type(PositionType::Absolute)
            .bottom(Val::Px(-80.));
        });

        out.insert(widget);
        out
    }
}
