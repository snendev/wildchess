use bevy::{
    prelude::{Component, In, Query, Reflect, ReflectComponent},
    utils::HashMap,
};

use crate::{
    actions::{Action, Actions},
    board::{Board, Square},
    pattern::Pattern,
    pieces::{Orientation, Position},
    team::Team,
};

use crate::behavior::Behavior;

#[derive(Clone, Debug, Default, Component, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct PatternBehavior {
    // in practice, this should rarely be more than one or two Patterns
    pub patterns: Vec<Pattern>,
}

impl PatternBehavior {
    pub fn new(patterns: Vec<Pattern>) -> Self {
        PatternBehavior { patterns }
    }

    pub fn join(mut self, mut other: Self) -> Self {
        self.patterns.append(&mut other.patterns);
        self
    }

    pub fn with_pattern(mut self, pattern: Pattern) -> Self {
        self.patterns.push(pattern);
        self
    }
}

// When a PatternBehavior runs a search, it must return a struct that contains
// the TargetMode (for visualization purposes)
impl PatternBehavior {
    pub(crate) fn search(
        &self,
        origin: &Square,
        orientation: &Orientation,
        my_team: &Team,
        board: &Board,
        pieces: &HashMap<Square, Team>,
        last_action: Option<&Action>,
    ) -> Actions {
        Actions::new(
            self.patterns
                .iter()
                .flat_map(|pattern| {
                    pattern.search(origin, orientation, my_team, board, pieces, last_action)
                })
                .collect(),
        )
    }
}

#[derive(Clone, Component, Debug)]
pub struct PatternActionsCache(Actions);

impl From<Actions> for PatternActionsCache {
    fn from(actions: Actions) -> Self {
        PatternActionsCache(actions)
    }
}

impl From<PatternActionsCache> for Actions {
    fn from(cache: PatternActionsCache) -> Self {
        cache.0
    }
}

impl Behavior for PatternBehavior {
    type ActionsCache = PatternActionsCache;

    fn calculate_actions_system(
        In(last_action): In<Option<Action>>,
        board_query: Query<&Board>,
        mut piece_query: Query<(
            Option<&PatternBehavior>,
            &Position,
            &Orientation,
            &Team,
            &mut PatternActionsCache,
        )>,
    ) {
        let Ok(board) = board_query.get_single() else {
            return;
        };
        let pieces: HashMap<Square, Team> = piece_query
            .iter()
            .map(|(_, position, _, team, _)| (position.0, *team))
            .collect();

        for (behavior, position, orientation, team, mut cache) in piece_query.iter_mut() {
            if let Some(behavior) = behavior {
                *cache = behavior
                    .search(
                        &position.0,
                        &orientation,
                        team,
                        board,
                        &pieces,
                        last_action.as_ref(),
                    )
                    .into();
            }
        }
    }
}
