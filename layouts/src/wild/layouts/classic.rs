use chess::{behavior::CastlingTarget, pieces::PieceDefinition};

use crate::{
    wild::pieces::{AdvancedBuilder, EliteBuilder, MajorBuilder, MinorBuilder, PieceBuilder},
    PieceSpecification,
};

use super::{king, pawn, piece, WildPieceSet};

pub struct ClassicWildLayout;

impl ClassicWildLayout {
    pub fn pieces() -> Vec<PieceSpecification> {
        // pieces
        let major: PieceDefinition = piece(MajorBuilder::random_behavior(), Some(CastlingTarget));
        let minor1 = piece(AdvancedBuilder::random_behavior(), None);
        let minor2 = piece(MinorBuilder::random_behavior(), None);
        let elite = piece(EliteBuilder::random_behavior(), None);

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
