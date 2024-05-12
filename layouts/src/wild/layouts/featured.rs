use chess::{behavior::CastlingTarget, pieces::PieceDefinition};

use crate::{
    wild::pieces::{
        AdvancedBuilder, EliteBuilder, KingBuilder, MajorBuilder, MinorBuilder, PawnBuilder,
    },
    PieceSpecification,
};

use super::{king, pawn, piece, WildPieceSet};

pub enum FeaturedWildLayout {
    One,
    Two,
    Three,
}

impl FeaturedWildLayout {
    pub fn pieces(self) -> Vec<PieceSpecification> {
        match self {
            FeaturedWildLayout::One => Self::one(),
            FeaturedWildLayout::Two => Self::two(),
            FeaturedWildLayout::Three => Self::three(),
        }
        .build_layout()
    }

    fn one() -> WildPieceSet {
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

        WildPieceSet {
            elite,
            major,
            minor1,
            minor2,
            pawn,
            king,
        }
    }

    fn two() -> WildPieceSet {
        // pieces
        let major: PieceDefinition = piece(AdvancedBuilder::ogre(), Some(CastlingTarget));
        let minor1 = piece(MajorBuilder::falconer(), None);
        let minor2 = piece(MinorBuilder::scorpion(), None);
        let elite = piece(EliteBuilder::executioner(), None);

        // pawns
        let pawn_promotion_options =
            vec![major.clone(), minor1.clone(), minor2.clone(), elite.clone()];
        let pawn: PieceDefinition = pawn(PawnBuilder::agile(), pawn_promotion_options);
        // king
        let king = king(KingBuilder::classical());

        WildPieceSet {
            elite,
            major,
            minor1,
            minor2,
            pawn,
            king,
        }
    }

    fn three() -> WildPieceSet {
        // pieces
        let major: PieceDefinition = piece(, Some(CastlingTarget));
        let minor1 = piece(, None);
        let minor2 = piece(, None);
        let elite = piece(EliteBuilder::chancellor(), None);

        // pawns
        let pawn_promotion_options =
            vec![major.clone(), minor1.clone(), minor2.clone(), elite.clone()];
        let pawn = pawn(PawnBuilder::torpedo(), pawn_promotion_options);
        // king
        let king = king(KingBuilder::frail());

        WildPieceSet {
            elite,
            major,
            minor1,
            minor2,
            pawn,
            king,
        }
    }
}
