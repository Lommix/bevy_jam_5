use crate::prelude::*;
use sickle_ui::prelude::*;

pub fn new_game(
    center: Query<Entity, With<TopUi>>,
    mut cmd: Commands,
    mut flow: ResMut<NextState<ControlFlow>>,
    sprites: Res<SpriteAssets>,
    items: Res<ItemAssets>,
) {
    let Ok(center_ui) = center.get_single() else {
        return;
    };

    let starting_food = cmd
        .spawn(ItemBundle {
            item: items.carrot.clone(),
            quantity: Quantity(40),
        })
        .id();

    let bag = cmd
        .spawn(Name::new("Player Inventory"))
        .add_child(starting_food)
        .id();

    let village_entity = cmd
        .spawn((
            GameSessionBundle {
                context: GameContext::default(),
                village_bundle: VillageBundle {
                    village: Village {
                        villager_count: 4,
                        villager_busy: 0,
                    },
                    inventory: Inventory::from_bag(bag),
                },
                ..default()
            },
            Name::new("Player"),
            Player,
            StateScoped(AppState::Playing),
        ))
        .id();

    cmd.ui_builder(center_ui)
        .div_centered(|div| {
            div.style().height(Val::Px(100.));
            div.village_hud(village_entity, &sprites);
        })
        .insert(StateScoped(AppState::Playing));

    flow.set(ControlFlow::Playing);
}
