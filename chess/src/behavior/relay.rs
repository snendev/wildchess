use bevy::{
    prelude::{Component, In, Query, Reflect, ReflectComponent},
    utils::HashMap,
};

use crate::{
    board::{Board, Square},
    pattern::Pattern,
    pieces::{Action, Actions, Orientation, Position},
    team::Team,
};

use super::{Behavior, PatternBehavior};

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

// Enable performing whatever Pattern was executed in the last turn
impl Behavior for RelayBehavior {
    fn add_actions_system(
        In(last_action): In<Option<Action>>,
        board_query: Query<&Board>,
        mut piece_query: Query<(
            Option<&RelayBehavior>,
            &Position,
            &Orientation,
            &Team,
            &mut Actions,
        )>,
    ) {
        let Ok(board) = board_query.get_single() else {
            return;
        };

        let pieces: HashMap<Square, Team> = piece_query
            .iter()
            .map(|(_, position, _, team, _)| (position.0, *team))
            .collect();

        // TODO: pre-filter this map so that it only stores the Squares with pieces on them
        // additionally, this could then only push patterns that match the appropriate team
        let mut relay_pattern_map: HashMap<Square, Vec<(Pattern, Team)>> = HashMap::new();

        for (relay_behavior, position, orientation, team, _) in piece_query.iter_mut() {
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

        for (_, position, orientation, team, mut actions) in piece_query.iter_mut() {
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
                actions.extend(PatternBehavior::new(patterns).search(
                    &position.0,
                    &orientation,
                    team,
                    board,
                    &pieces,
                    last_action.as_ref(),
                ))
            }
        }
    }
}
