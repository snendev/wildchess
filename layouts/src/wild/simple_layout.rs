use chess::board::{File, Rank, Square};

use crate::{wild::simple_random_pieces, PieceSpecification};

pub struct SimpleWildLayout;

impl SimpleWildLayout {
    pub fn pieces() -> Vec<PieceSpecification> {
        let piece_set = simple_random_pieces();
        [File::A, File::H]
            .into_iter()
            .map(move |file| {
                PieceSpecification::new(piece_set.major.clone(), Square::new(file, Rank::ONE))
            })
            .chain([File::B, File::G].into_iter().map(move |file| {
                PieceSpecification::new(piece_set.minor1.clone(), Square::new(file, Rank::ONE))
            }))
            .chain([File::C, File::F].into_iter().map(move |file| {
                PieceSpecification::new(piece_set.minor2.clone(), Square::new(file, Rank::ONE))
            }))
            .chain(std::iter::once(File::D).map(move |file| {
                PieceSpecification::new(piece_set.elite.clone(), Square::new(file, Rank::ONE))
            }))
            .chain(std::iter::once(File::E).map(move |file| {
                PieceSpecification::new(piece_set.king.clone(), Square::new(file, Rank::ONE))
            }))
            .chain((0..8).map(File::from).map(move |file| {
                PieceSpecification::new(piece_set.pawn.clone(), Square::new(file, Rank::TWO))
            }))
            .collect()
    }
}
