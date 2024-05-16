use rand::{thread_rng, Rng};

use chess::{
    behavior::CastlingTarget,
    pieces::{PieceDefinition, PieceIdentity},
};

use crate::{wild::pieces::PieceBuilder, PieceSpecification};

use super::{king, pawn, piece, WildPieceSet};

pub struct RandomWildLayout;

impl RandomWildLayout {
    pub fn pieces() -> Vec<PieceSpecification> {
        let mut rng = thread_rng();
        let max_value: u32 = rng.gen_range(50..80);
        let mut current_value: u32 = 0;

        // pieces
        let major: PieceDefinition = piece(
            PieceBuilder::random_behavior(max_value, &mut current_value),
            PieceIdentity::Rook,
            Some(CastlingTarget),
        );
        let minor1 = piece(
            PieceBuilder::random_behavior(max_value, &mut current_value),
            PieceIdentity::Knight,
            None,
        );
        let minor2 = piece(
            PieceBuilder::random_behavior(max_value, &mut current_value),
            PieceIdentity::Bishop,
            None,
        );
        let elite = piece(
            PieceBuilder::random_behavior(max_value, &mut current_value),
            PieceIdentity::Queen,
            None,
        );

        // pawns
        let pawn_promotion_options =
            vec![major.clone(), minor1.clone(), minor2.clone(), elite.clone()];
        let pawn = pawn(PieceBuilder::generate_pawn(), pawn_promotion_options);
        // king
        let king = king(PieceBuilder::generate_king());

        let piece_set = WildPieceSet {
            elite,
            major,
            minor1,
            minor2,
            pawn,
            king,
        };
        piece_set.build_layout()
    }
}
