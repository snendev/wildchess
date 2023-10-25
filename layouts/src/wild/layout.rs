use chess::{
    board::{Board, File},
    pieces::PieceSpecification,
};

use crate::{utils::squares_by_team, wild::random_pieces};

pub struct WildLayout;

impl WildLayout {
    pub fn pieces<'a>(board: &'a Board) -> impl Iterator<Item = PieceSpecification> + 'a {
        let piece_set = random_pieces();

        squares_by_team(0, board, [File::A, File::H].into_iter())
            .map(move |(team, square)| {
                PieceSpecification::new(piece_set.major.clone(), team, square.into())
            })
            .chain(
                squares_by_team(0, board, [File::B, File::G].into_iter()).map(
                    move |(team, square)| {
                        PieceSpecification::new(piece_set.minor1.clone(), team, square.into())
                    },
                ),
            )
            .chain(
                squares_by_team(0, board, [File::C, File::F].into_iter()).map(
                    move |(team, square)| {
                        PieceSpecification::new(piece_set.minor2.clone(), team, square.into())
                    },
                ),
            )
            .chain(squares_by_team(0, board, std::iter::once(File::D)).map(
                move |(team, square)| {
                    PieceSpecification::new(piece_set.elite.clone(), team, square.into())
                },
            ))
            .chain(squares_by_team(0, board, std::iter::once(File::E)).map(
                move |(team, square)| {
                    PieceSpecification::new(piece_set.king.clone(), team, square.into())
                },
            ))
            .chain(
                squares_by_team(1, board, (0..8).map(File::from)).map(move |(team, square)| {
                    PieceSpecification::new(piece_set.pawn.clone(), team, square.into())
                }),
            )
    }
}
