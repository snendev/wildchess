use chess::{behavior::CastlingTarget, pieces::PieceDefinition};

use crate::{
    wild::pieces::{
        AdvancedBuilder, EliteBuilder, KingBuilder, MajorBuilder, MinorBuilder, PawnBuilder,
    },
    PieceSpecification,
};

use super::{king, pawn, piece, WildPieceSet};

pub struct FeaturedWildLayout;

impl FeaturedWildLayout {
    pub fn pieces() -> Vec<PieceSpecification> {
        // pieces
        let major: PieceDefinition = piece(MajorBuilder::butterfly(), Some(CastlingTarget));
        let minor1 = piece(MinorBuilder::prince(), None);
        let minor2 = piece(AdvancedBuilder::bishop(), None);
        let elite = piece(EliteBuilder::panther(), None);

        // pawns
        let pawn_promotion_options =
            vec![major.clone(), minor1.clone(), minor2.clone(), elite.clone()];
        let pawn = pawn(PawnBuilder::historic(), pawn_promotion_options);
        // king
        let king = king(KingBuilder::gold_general());

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
