#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
use bevy_ecs::prelude::{Changed, Component, Entity, Query, With};
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;
use bevy_utils::{HashMap, HashSet};

use fairy_gameboard::GameBoard;

use crate::{actions::Actions, pieces::Position, team::Team, ChessBoard, OnBoard};

#[derive(Clone, Debug, Default)]
#[derive(Component)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub struct BoardPieceCache<B: GameBoard> {
    entities: HashMap<Entity, B::Vector>,
    pub teams: HashMap<B::Vector, Team>,
}

impl<B: GameBoard> BoardPieceCache<B> {
    pub(crate) fn track_pieces(
        mut board_query: Query<&mut Self, With<ChessBoard<B>>>,
        // Actions should change every move for all pieces
        piece_query: Query<(Entity, &OnBoard, &Team, &Position<B>), Changed<Position<B>>>,
    ) where
        B: Send + Sync + 'static,
    {
        for (piece, on_board, team, position) in piece_query.iter() {
            let Ok(mut cache) = board_query.get_mut(on_board.0) else {
                continue;
            };

            let mut prev_position = None;
            if let Some(piece_position) = cache.entities.get_mut(&piece) {
                prev_position = Some(*piece_position);
                *piece_position = *position;
            }
            if let Some(prev_position) = prev_position {
                cache.teams.remove(&prev_position);
            }

            cache.entities.insert(piece, *position);
            cache.teams.insert(*position, *team);
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct BoardThreat<B: GameBoard> {
    position: B::Vector,
    attacked_team: Team,
}

#[derive(Clone, Debug, Default)]
#[derive(Component)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub struct BoardThreatsCache<B: GameBoard>(HashSet<BoardThreat<B>>);

impl<B: GameBoard> BoardThreatsCache<B> {
    pub(crate) fn is_threatened(&self, position: B::Vector, team: Team) -> bool {
        self.0.contains(&BoardThreat {
            position,
            attacked_team: team,
        })
    }

    pub(crate) fn track_pieces(
        mut board_query: Query<&mut Self, With<ChessBoard<B>>>,
        // Actions should change every move for all pieces
        piece_query: Query<(Entity, &OnBoard, &Team, &Actions<B>), Changed<Actions<B>>>,
    ) where
        B: Send + Sync + 'static,
    {
        // first clear everything
        let mut affected_boards = HashSet::new();
        for (_piece, on_board, _, _) in piece_query.iter() {
            affected_boards.insert(on_board.0);
        }
        for board in affected_boards {
            let Ok(mut attacked_positions) = board_query.get_mut(board) else {
                continue;
            };
            attacked_positions.0.clear();
        }

        // then instantiate the maps
        for (_piece, on_board, team, actions) in piece_query.iter() {
            let Ok(mut attacked_positions) = board_query.get_mut(on_board.0) else {
                continue;
            };
            for &capture_position in actions.0.iter().flat_map(|(_, action)| &action.threats) {
                attacked_positions.0.insert(BoardThreat {
                    position: capture_position,
                    attacked_team: match team {
                        Team::White => Team::Black,
                        Team::Black => Team::White,
                    },
                });
            }
        }
    }
}
