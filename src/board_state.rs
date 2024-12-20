use serde::{Deserialize, Serialize};

use bevy::{
    prelude::{Component, Query, With},
    utils::HashMap,
};

use games::{
    chess::pieces::{HasPieces, Position},
    components::{CurrentTurn, HasPlayers, Player},
};
use wild_icons::PieceIconSource;

use crate::{
    games::{
        chess::{
            actions::LastAction,
            board::{Board, Square},
            pieces::{Mutation, PieceIdentity},
            team::Team,
        },
        Clock,
    },
    wild_icons::PieceIconSvg,
};

#[derive(Clone, Debug)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
pub struct BoardState {
    pub size: (u16, u16),
    pub current_turn: Team,
    pub pieces: PieceMap,
    pub icons: PieceIconMap,
    pub clocks: Vec<Clock>,
    pub last_action: Option<LastAction>,
}

impl BoardState {
    pub fn track_state(
        mut boards: Query<(
            &Board,
            Option<&mut BoardState>,
            Option<&LastAction>,
            &HasPlayers,
            &HasPieces,
        )>,
        players: Query<(&Clock, &Team, Option<&CurrentTurn>), With<Player>>,
        pieces: Query<(
            &PieceIdentity,
            &Position,
            &Team,
            Option<&Mutation>,
            &PieceIconSvg,
        )>,
    ) {
        for (board, state, last_action, board_players, board_pieces) in boards.iter_mut() {
            let size = (board.size.file.0, board.size.rank.0);
            let current_turn = if let Some(team) = players
                .get(board_players.0)
                .ok()
                .and_then(|(_, team, turn)| turn.map(|_| *team))
            {
                team
            } else if let Some(team) = players
                .get(board_players.1)
                .ok()
                .and_then(|(_, team, turn)| turn.map(|_| *team))
            {
                team
            } else {
                bevy::log::warn!("Board exists with invalid state: no player has a turn?");
                continue;
            };

            let piece_map = PieceMap(
                board_pieces
                    .0
                    .iter()
                    .filter_map(|piece| {
                        pieces
                            .get(*piece)
                            .map(|(piece, position, team, mutation, _)| {
                                (position.0, (*piece, *team, mutation.cloned()))
                            })
                            .inspect_err(|err| bevy::log::warn!("Board expects piece but piece is not found by BoardState query: {err}"))
                            .ok()
                    })
                    .collect(),
            );
            let icons = PieceIconMap(
                pieces
                    .iter()
                    .map(|(piece, _, _, _, icon)| (*piece, icon.source.clone()))
                    .collect(),
            );
            let clocks = vec![
                players.get(board_players.0).unwrap().0.clone(),
                players.get(board_players.1).unwrap().0.clone(),
            ];
            let next_state = BoardState {
                size,
                current_turn,
                pieces: piece_map,
                icons,
                clocks,
                last_action: last_action.cloned(),
            };
            if let Some(mut state) = state {
                *state = next_state;
            }
        }
    }
}

impl Default for BoardState {
    fn default() -> Self {
        Self {
            size: (8, 8),
            current_turn: Default::default(),
            pieces: Default::default(),
            icons: Default::default(),
            clocks: Default::default(),
            last_action: None,
        }
    }
}

#[derive(Clone, Debug, Default)]
#[derive(Deserialize, Serialize)]
pub struct PieceMap(pub HashMap<Square, (PieceIdentity, Team, Option<Mutation>)>);

#[derive(Clone, Debug, Default)]
#[derive(Deserialize, Serialize)]
pub struct PieceIconMap(pub HashMap<PieceIdentity, PieceIconSource>);
