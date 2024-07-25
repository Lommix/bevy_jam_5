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
    food: Query<&Quantity, With<Eatable>>,
    seasons: Res<State<Season>>,
    mut texts: Query<&mut Text>,
) {
    huds.iter().for_each(|hud| {
        let Ok((village, inventory)) = villages.get(hud.village)
        else {
            return;
        };

        let Ok(items) = bag.get(inventory.bag) else {
            return;
        };

        let food_count = items
            .iter()
            .flat_map(|ent| food.get(*ent).ok())
            .map(|quant| **quant)
            .sum::<i32>();

        _ = texts.get_mut(hud.season_txt).map(|mut txt| {
            let s = seasons.get().to_string();
            txt.sections = vec![s.into()];
        });

        _ = texts.get_mut(hud.food_txt).map(|mut txt| {
            txt.sections =
                vec![format!("{} Food", food_count).into()];
        });
        _ = texts.get_mut(hud.gold_txt).map(|mut txt| {
            txt.sections =
                vec![format!("{} Gold", inventory.gold).into()];
        });
        _ = texts.get_mut(hud.villager_count_txt).map(|mut txt| {
            txt.sections =
                vec![format!("{} Villager", village.villager_count)
                    .into()];
        });
    });
}

#[derive(Component, UiContext)]
pub struct VillageHudWidget {
    village: Entity,
    food_txt: Entity,
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
            .bottom(Val::Px(-40.));
        });

        out.insert(widget);
        out
    }
}
