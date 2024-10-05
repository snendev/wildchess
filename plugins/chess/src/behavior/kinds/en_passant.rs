use serde::{Deserialize, Serialize};

use bevy::prelude::{Commands, Component, Entity, Query, Reflect};
use bevy::utils::HashMap;

use crate::{
    actions::{Action, Actions, LastAction},
    behavior::BoardPieceCache,
    board::{Board, OnBoard, Square},
    pattern::Pattern,
    pieces::{Orientation, Position},
    team::Team,
};

use crate::behavior::Behavior;

#[derive(Clone, Copy, Debug, Default)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct EnPassantBehavior;

#[derive(Clone, Debug)]
#[derive(Component, Reflect)]
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
                        .iter()
                        .next()
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
        mut commands: Commands,
        board_query: Query<(Entity, &Board, &BoardPieceCache, Option<&LastAction>)>,
        mut piece_query: Query<(
            Entity,
            Option<&EnPassantBehavior>,
            Option<&mut EnPassantActionsCache>,
            &Position,
            &Orientation,
            &Team,
            &OnBoard,
        )>,
    ) {
        for (board_entity, board, _pieces, last_action) in board_query.iter() {
            let en_passant_pieces = piece_query
                .iter()
                .filter(|(_, _, _, _, _, _, on_board)| on_board.0 == board_entity)
                .map(|(_, en_passant, _, position, _, team, _)| {
                    (position.0, (en_passant.copied(), *team))
                })
                .collect::<HashMap<_, _>>();

            for (entity, behavior, cache, position, orientation, team, _) in piece_query
                .iter_mut()
                .filter(|(_, _, _, _, _, _, on_board)| on_board.0 == board_entity)
            {
                if let Some(behavior) = behavior {
                    let actions = EnPassantActionsCache::from(behavior.search(
                        &position.0,
                        orientation,
                        team,
                        board,
                        &en_passant_pieces,
                        last_action.map(|action: &LastAction| &action.0),
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

#[cfg(test)]
mod tests {
    use bevy::utils::HashSet;

    use crate::{
        actions::Movement,
        board::{File, Rank},
    };

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

    fn previous_move_square() -> Square {
        Square::new(File::B, Rank::TWO)
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
            previous_move_square(),
            en_passant_capture_square(),
            Orientation::Down,
            vec![Square::new(File::B, Rank::FIVE)],
            Some(Pattern::forward()),
        )
    }

    fn en_passant_action() -> Action {
        let mut captures = HashSet::new();
        captures.insert(en_passant_capture_square());
        let mut threats = HashSet::new();
        threats.insert(en_passant_target_square());
        Action {
            movement: Movement::new(origin(), en_passant_target_square(), Orientation::Up),
            using_pattern: Some(Pattern::en_passant()),
            captures,
            threats,
            ..Default::default()
        }
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
