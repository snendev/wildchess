use serde::{Deserialize, Serialize};

#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
use bevy_ecs::prelude::{Commands, Component, Entity, Query};
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::Reflect;
use bevy_utils::HashMap;

use crate::{
    actions::{Action, Actions, LastAction},
    behavior::BoardPieceCache,
    board::{Board, OnBoard, Square},
    pattern::Pattern,
    pieces::{Orientation, Position},
    team::Team,
};

use crate::behavior::Behavior;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
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

#[derive(Clone, Debug)]
#[derive(Component)]
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
        mut commands: Commands,
        board_query: Query<(Entity, &Board, &BoardPieceCache, Option<&LastAction>)>,
        mut piece_query: Query<(
            Entity,
            Option<&PatternBehavior>,
            Option<&mut PatternActionsCache>,
            &Position,
            &Orientation,
            &Team,
            &OnBoard,
        )>,
    ) {
        for (board_entity, board, pieces, last_action) in board_query.iter() {
            for (entity, behavior, cache, position, orientation, team, _) in piece_query
                .iter_mut()
                .filter(|(_, _, _, _, _, _, on_board)| on_board.0 == board_entity)
            {
                if let Some(behavior) = behavior {
                    let actions = PatternActionsCache::from(behavior.search(
                        &position.0,
                        orientation,
                        team,
                        board,
                        &pieces.teams,
                        last_action.map(|action| &action.0),
                    ));
                    if let Some(mut cache) = cache {
                        *cache = actions;
                    } else {
                        commands.entity(entity).insert(actions);
                    }
                }
            }
        }
    }
}
