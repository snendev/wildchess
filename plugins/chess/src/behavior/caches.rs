#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
use bevy_ecs::prelude::{Changed, Component, Entity, Query, With};
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;
use bevy_utils::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::{
    actions::Actions,
    board::{Board, OnBoard, Square},
    pieces::Position,
    team::Team,
};

#[derive(Clone, Debug, Default)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub struct BoardPieceCache {
    entities: HashMap<Entity, Square>,
    pub teams: HashMap<Square, Team>,
}

impl BoardPieceCache {
    pub(crate) fn track_pieces(
        mut board_query: Query<&mut Self, With<Board>>,
        // Actions should change every move for all pieces
        piece_query: Query<(Entity, &OnBoard, &Team, &Position), Changed<Position>>,
    ) {
        for (piece, on_board, team, position) in piece_query.iter() {
            let Ok(mut cache) = board_query.get_mut(on_board.0) else {
                continue;
            };

            let mut prev_square = None;
            if let Some(square) = cache.entities.get_mut(&piece) {
                prev_square = Some(*square);
                *square = position.0;
            }
            if let Some(prev_square) = prev_square {
                cache.teams.remove(&prev_square);
            }

            cache.entities.insert(piece, position.0);
            cache.teams.insert(position.0, *team);
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct BoardThreat {
    square: Square,
    attacked_team: Team,
}

#[derive(Clone, Debug, Default)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub struct BoardThreatsCache(HashSet<BoardThreat>);

impl BoardThreatsCache {
    pub(crate) fn is_threatened(&self, square: Square, team: Team) -> bool {
        self.0.contains(&BoardThreat {
            square,
            attacked_team: team,
        })
    }

    pub(crate) fn track_pieces(
        mut board_query: Query<&mut Self, With<Board>>,
        // Actions should change every move for all pieces
        piece_query: Query<(Entity, &OnBoard, &Team, &Actions), Changed<Actions>>,
    ) {
        // first clear everything
        let mut affected_boards = HashSet::new();
        for (_piece, on_board, _, _) in piece_query.iter() {
            affected_boards.insert(on_board.0);
        }
        for board in affected_boards {
            let Ok(mut attacked_squares) = board_query.get_mut(board) else {
                continue;
            };
            attacked_squares.0.clear();
        }

        // then instantiate the maps
        for (_piece, on_board, team, actions) in piece_query.iter() {
            let Ok(mut attacked_squares) = board_query.get_mut(on_board.0) else {
                continue;
            };
            for &capture_square in actions.0.iter().flat_map(|(_, action)| &action.threats) {
                attacked_squares.0.insert(BoardThreat {
                    square: capture_square,
                    attacked_team: match team {
                        Team::White => Team::Black,
                        Team::Black => Team::White,
                    },
                });
            }
        }
    }
}
