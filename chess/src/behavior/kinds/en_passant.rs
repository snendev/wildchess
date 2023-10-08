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

use crate::behavior::Behavior;

#[derive(Clone, Copy, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct EnPassantBehavior;

#[derive(Clone, Component, Debug)]
pub struct EnPassantActionsCache(Actions);

impl From<Actions> for EnPassantActionsCache {
    fn from(actions: Actions) -> Self {
        EnPassantActionsCache(actions)
    }
}

impl From<EnPassantActionsCache> for Actions {
    fn from(cache: EnPassantActionsCache) -> Self {
        cache.0
    }
}

// store last turn
// detect whether the piece has "stepped" on an attackable square

impl EnPassantBehavior {
    fn search(
        &self,
        origin: &Square,
        orientation: &Orientation,
        my_team: &Team,
        board: &Board,
        pieces: &HashMap<Square, (Option<EnPassantBehavior>, Team)>,
        last_action: Option<&Action>,
    ) -> Actions {
        let capture_in_passing_search = Pattern::en_passant().search(
            origin,
            orientation,
            my_team,
            board,
            &pieces
                .iter()
                .map(|(square, (_, team))| (*square, *team))
                .collect(),
            last_action,
        );

        Actions::new(
            capture_in_passing_search
                .into_iter()
                .filter_map(|(square, action)| {
                    action
                        .captures
                        .first()
                        .and_then(|capture| pieces.get(capture))
                        .and_then(|(en_passant, _)| *en_passant)
                        .map(|_| (square, action))
                })
                .collect(),
        )
    }
}

impl Behavior for EnPassantBehavior {
    type ActionsCache = EnPassantActionsCache;

    fn calculate_actions_system(
        In(last_action): In<Option<Action>>,
        mut commands: Commands,
        board_query: Query<&Board>,
        mut piece_query: Query<(
            Entity,
            Option<&EnPassantBehavior>,
            Option<&mut EnPassantActionsCache>,
            &Position,
            &Orientation,
            &Team,
        )>,
    ) {
        let Ok(board) = board_query.get_single() else {
            return;
        };

        let en_passant_pieces = piece_query
            .iter()
            .map(|(_, en_passant, _, position, _, team)| {
                (position.0, (en_passant.copied(), *team))
            })
            .collect::<HashMap<_, _>>();

        for (entity, behavior, cache, position, orientation, team) in piece_query.iter_mut() {
            if let Some(behavior) = behavior {
                let actions = EnPassantActionsCache::from(behavior.search(
                    &position.0,
                    orientation,
                    team,
                    board,
                    &en_passant_pieces,
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

#[cfg(test)]
mod test {
    use crate::board::{File, Rank};

    use super::*;

    fn origin() -> Square {
        Square::new(File::C, Rank::FOUR)
    }

    fn en_passant_target_square() -> Square {
        Square::new(File::B, Rank::FIVE)
    }

    fn en_passant_capture_square() -> Square {
        Square::new(File::B, Rank::FOUR)
    }

    fn en_passant_scenario_board(
        attackable_piece_team: Team,
    ) -> HashMap<Square, (Option<EnPassantBehavior>, Team)> {
        let mut map = HashMap::new();
        map.insert(origin(), (Some(EnPassantBehavior), Team::White));
        map.insert(
            Square::new(File::B, Rank::FOUR),
            (Some(EnPassantBehavior), attackable_piece_team),
        );
        map
    }

    fn not_en_passant_scenario_board() -> HashMap<Square, (Option<EnPassantBehavior>, Team)> {
        let mut map = HashMap::new();
        map.insert(origin(), (Some(EnPassantBehavior), Team::White));
        map.insert(Square::new(File::B, Rank::FOUR), (None, Team::Black));
        map
    }

    fn last_action() -> Action {
        Action::movement(
            en_passant_capture_square(),
            Orientation::Down,
            vec![Square::new(File::B, Rank::FIVE)],
            Pattern::forward(),
        )
    }

    fn en_passant_action() -> Action {
        Action::capture(
            en_passant_target_square(),
            Orientation::Up,
            vec![],
            Pattern::en_passant(),
            vec![en_passant_capture_square()],
        )
    }

    #[test]
    fn test_en_passant_activation() {
        let results = EnPassantBehavior.search(
            &origin(),
            &Orientation::Up,
            &Team::White,
            &Board::chess_board(),
            &en_passant_scenario_board(Team::Black),
            Some(&last_action()),
        );

        assert_eq!(
            results.get(&en_passant_target_square()),
            Some(&en_passant_action()),
            "En passant failed: {:?}",
            results
        );
    }

    #[test]
    fn test_en_passant_wrong_team() {
        let results = EnPassantBehavior.search(
            &origin(),
            &Orientation::Up,
            &Team::White,
            &Board::chess_board(),
            &en_passant_scenario_board(Team::White),
            Some(&last_action()),
        );

        assert_eq!(
            results.get(&en_passant_target_square()),
            None,
            "En passant failed: {:?}",
            results
        );
    }

    #[test]
    fn test_en_passant_wrong_piece() {
        let results = EnPassantBehavior.search(
            &origin(),
            &Orientation::Up,
            &Team::White,
            &Board::chess_board(),
            &not_en_passant_scenario_board(),
            Some(&last_action()),
        );

        assert_eq!(
            results.get(&en_passant_target_square()),
            None,
            "En passant failed: {:?}",
            results
        );
    }
}
