use std::marker::PhantomData;

#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
use bevy_ecs::prelude::{Commands, Component, Entity, In, Query};
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::Reflect;
use bevy_utils::HashMap;

use fairy_gameboard::{BoardVector, GameBoard};

use crate::{
    actions::{Action, Actions},
    behavior::BoardPieceCache,
    pattern::Pattern,
    pieces::{Orientation, Position},
    team::Team,
    ChessBoard,
};

use crate::behavior::Behavior;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[derive(Component)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct PatternBehavior<B: GameBoard> {
    // in practice, this should rarely be more than one or two Patterns
    pub patterns: Vec<Pattern<B>>,
    // TODO: why is above B usage insufficient?
    marker: PhantomData<B>,
}

impl<B: GameBoard> PatternBehavior<B> {
    pub fn new(patterns: Vec<Pattern<B>>) -> Self {
        PatternBehavior {
            patterns,
            marker: PhantomData::<B>,
        }
    }

    pub fn join(mut self, mut other: Self) -> Self {
        self.patterns.append(&mut other.patterns);
        self
    }

    pub fn with_pattern(mut self, pattern: Pattern<B>) -> Self {
        self.patterns.push(pattern);
        self
    }

    pub fn add_pattern(&mut self, pattern: Pattern<B>) {
        self.patterns.push(pattern);
    }
}

// When a PatternBehavior runs a search, it must return a struct that contains
// the TargetMode (for visualization purposes)
impl<B: GameBoard> PatternBehavior<B> {
    pub(crate) fn search(
        &self,
        origin: &B::Vector,
        orientation: &<B::Vector as BoardVector>::Symmetry,
        my_team: &Team,
        board: &B,
        pieces: &HashMap<B::Vector, Team>,
        last_action: Option<&Action<B>>,
    ) -> Actions<B> {
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
pub struct PatternActionsCache<B: GameBoard>(Actions<B>);

impl<B: GameBoard> From<Actions<B>> for PatternActionsCache<B> {
    fn from(actions: Actions<B>) -> Self {
        PatternActionsCache(actions)
    }
}

impl<B: GameBoard> From<PatternActionsCache<B>> for Actions<B> {
    fn from(cache: PatternActionsCache<B>) -> Self {
        cache.0
    }
}

impl<B> Behavior<B> for PatternBehavior<B>
where
    B: GameBoard + Send + Sync + 'static,
{
    type ActionsCache = PatternActionsCache<B>;

    fn calculate_actions_system(
        mut commands: Commands,
        board_query: Query<(&ChessBoard<B>, &BoardPieceCache<B>)>,
        mut piece_query: Query<(
            Entity,
            Option<&PatternBehavior<B>>,
            Option<&mut PatternActionsCache<B>>,
            &Position<B>,
            &Orientation<B>,
            &Team,
        )>,
    ) {
        let Ok((board, pieces)) = board_query.get_single() else {
            return;
        };

        for (entity, behavior, cache, position, orientation, team) in piece_query.iter_mut() {
            if let Some(behavior) = behavior {
                let actions = PatternActionsCache::from(behavior.search(
                    &*position,
                    orientation,
                    team,
                    board,
                    &pieces.teams,
                    // TODO
                    None, // last_action.as_ref(),
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
