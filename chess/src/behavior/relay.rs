use bevy::{
    prelude::{Component, In, Query, Reflect, ReflectComponent},
    utils::HashMap,
};

use crate::{
    board::{Board, Square},
    pieces::{Action, Actions, Orientation, Pattern, Position},
    team::Team,
};

use super::{Behavior, PatternBehavior};

#[derive(Clone, Component, Debug, Default, Reflect)]
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

        let mut relay_pattern_map: HashMap<Square, Vec<Pattern>> = HashMap::new();
        for (relay_behavior, position, orientation, team, _) in piece_query.iter_mut() {
            if let Some(relay_behavior) = relay_behavior {
                for pattern in relay_behavior.patterns.iter() {
                    for scan_target in
                        pattern
                            .scanner
                            .scan(&position.0, *orientation, team, board, &pieces)
                    {
                        if let Some(patterns) = relay_pattern_map.get_mut(&scan_target.target) {
                            patterns.push(pattern.clone());
                        } else {
                            relay_pattern_map.insert(scan_target.target, vec![pattern.clone()]);
                        }
                    }
                }
            }
        }

        for (_, position, orientation, team, mut actions) in piece_query.iter_mut() {
            if let Some(patterns) = relay_pattern_map.remove(&position.0) {
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
