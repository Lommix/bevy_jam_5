use crate::prelude::*;

pub struct SeasonPlugin;
impl Plugin for SeasonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SeasonShiftEvent>()
            .init_state::<Season>()
            .enable_state_scoped_entities::<Season>()
            .add_systems(OnExit(ControlFlow::Autoplay), next_season);
    }
}

#[derive(Event)]
pub struct SeasonShiftEvent {
    pub from: Season,
    pub to: Season,
}

fn next_season(
    mut events: EventWriter<SeasonShiftEvent>,
    mut game_sessions: Query<&mut GameContext>,
    state: Res<State<Season>>,
    mut next: ResMut<NextState<Season>>,
) {
    game_sessions.iter_mut().for_each(|mut context| {
        let last_season = state.get().clone();
        let next_season = last_season.next();

        context.current_turn += 1;

        next.set(next_season);
        events.send(SeasonShiftEvent {
            from: last_season,
            to: next_season,
        });
    });
}

#[derive(
    Hash, Reflect, PartialEq, Eq, Debug, Default, Clone, Copy, States,
)]
#[reflect]
pub enum Season {
    #[default]
    Spring,
    Summer,
    Autum,
    Winter,
}

impl Season {
    pub fn next(self) -> Self {
        match self {
            Season::Spring => Season::Summer,
            Season::Summer => Season::Autum,
            Season::Autum => Season::Winter,
            Season::Winter => Season::Spring,
        }
    }

    pub fn to_string(self) -> String {
        match self {
            Season::Spring => "Spring".into(),
            Season::Summer => "Summer".into(),
            Season::Autum => "Autum".into(),
            Season::Winter => "Winter".into(),
        }
    }

    pub fn animation(&self) -> &str {
        match self {
            Season::Spring => "spring",
            Season::Summer => "summer",
            Season::Autum => "autum",
            Season::Winter => "winter",
        }
    }
}
