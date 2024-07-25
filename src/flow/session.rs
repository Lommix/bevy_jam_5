use crate::prelude::*;

#[derive(Bundle)]
pub struct GameSessionBundle {
    pub context: GameContext,
    pub village_bundle: VillageBundle,
}

impl Default for GameSessionBundle {
    fn default() -> Self {
        GameSessionBundle {
            context: GameContext::default(),
            village_bundle: VillageBundle {
                village: Village {
                    villager_count: 4,
                    villager_busy: 0,
                },
                inventory: Default::default(),
            },
        }
    }
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect]
pub struct GameContext {
    pub current_turn: u32,
}

impl GameContext {
    pub fn current_year(&self) -> u32 {
        self.current_turn / 4
    }
}
