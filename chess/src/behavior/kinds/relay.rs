use bevy::{
    prelude::{Commands, Component, Entity, In, Query, Reflect, ReflectComponent},
    utils::HashMap,
};

use crate::{
    actions::{Action, Actions},
    board::{Board, Square},
    pattern::Pattern,
    pieces::{Orientation, Position},
    team::Team,
};

use crate::behavior::{Behavior, PatternBehavior};

#[derive(Clone, Debug, Default, Component, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
// A behavior that "relays" patterns to scanned ally pieces
pub struct RelayBehavior {
    pub patterns: Vec<Pattern>,
}

impl From<PatternBehavior> for RelayBehavior {
    fn from(behavior: PatternBehavior) -> Self {
        RelayBehavior {
            patterns: behavior.patterns,
        }
    }
}

#[derive(Clone, Component, Debug)]
pub struct RelayActionsCache(Actions);

impl From<Actions> for RelayActionsCache {
    fn from(actions: Actions) -> Self {
        RelayActionsCache(actions)
    }
}

impl From<RelayActionsCache> for Actions {
    fn from(cache: RelayActionsCache) -> Self {
        cache.0
    }
}

// Enable performing whatever Pattern was executed in the last turn
impl Behavior for RelayBehavior {
    type ActionsCache = RelayActionsCache;

    fn calculate_actions_system(
        In(last_action): In<Option<Action>>,
        mut commands: Commands,
        board_query: Query<&Board>,
        mut piece_query: Query<(
            Entity,
            Option<&RelayBehavior>,
            Option<&mut RelayActionsCache>,
            &Position,
            &Orientation,
            &Team,
        )>,
    ) {
        let Ok(board) = board_query.get_single() else {
            return;
        };

        let pieces: HashMap<Square, Team> = piece_query
            .iter()
            .map(|(_, _, _, position, _, team)| (position.0, *team))
            .collect();

        // TODO: pre-filter this map so that it only stores the Squares with pieces on them
        // additionally, this could then only push patterns that match the appropriate team
        let mut relay_pattern_map: HashMap<Square, Vec<(Pattern, Team)>> = HashMap::new();

        for (_, relay_behavior, _, position, orientation, team) in piece_query.iter_mut() {
            if let Some(relay_behavior) = relay_behavior {
                for pattern in relay_behavior.patterns.iter() {
                    for scan_target in
                        pattern
                            .scanner
                            .scan(&position.0, *orientation, team, board, &pieces)
                    {
                        if let Some(patterns) = relay_pattern_map.get_mut(&scan_target.target) {
                            patterns.push((pattern.clone(), *team));
                        } else {
                            relay_pattern_map
                                .insert(scan_target.target, vec![(pattern.clone(), *team)]);
                        }
                    }
                }
            }
        }

        for (entity, _, cache, position, orientation, team) in piece_query.iter_mut() {
            if let Some(patterns) = relay_pattern_map.remove(&position.0) {
                let patterns = patterns
                    .into_iter()
                    .filter_map(|(pattern, source_team)| {
                        if *team == source_team {
                            Some(pattern)
                        } else {
                            None
                        }
                    })
                    .collect();
                let actions = RelayActionsCache::from(PatternBehavior::new(patterns).search(
                    &position.0,
                    &orientation,
                    team,
                    board,
                    &pieces,
                    last_action.as_ref(),
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
