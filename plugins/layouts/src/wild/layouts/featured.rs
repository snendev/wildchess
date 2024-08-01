use chess::{
    behavior::CastlingTarget,
    pieces::{PieceDefinition, PieceIdentity},
};

use crate::{
    wild::pieces::{
        AdvancedBuilder, EliteBuilder, InfantryBuilder, KingBuilder, MajorBuilder, MinorBuilder,
        PawnBuilder,
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
        let major: PieceDefinition = piece(
            MajorBuilder::butterfly(),
            PieceIdentity::Rook,
            Some(CastlingTarget),
        );
        let minor1 = piece(MinorBuilder::prince(), PieceIdentity::Knight, None);
        let minor2 = piece(AdvancedBuilder::bishop(), PieceIdentity::Bishop, None);
        let elite = piece(EliteBuilder::panther(), PieceIdentity::Queen, None);

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
        let major: PieceDefinition = piece(
            AdvancedBuilder::ogre(),
            PieceIdentity::Rook,
            Some(CastlingTarget),
        );
        let minor1 = piece(MajorBuilder::falconer(), PieceIdentity::Knight, None);
        let minor2 = piece(MinorBuilder::scorpion(), PieceIdentity::Bishop, None);
        let elite = piece(EliteBuilder::executioner(), PieceIdentity::Queen, None);

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
        let major: PieceDefinition = piece(
            InfantryBuilder::raven(),
            PieceIdentity::Rook,
            Some(CastlingTarget),
        );
        let minor1 = piece(MajorBuilder::cardinal(), PieceIdentity::Knight, None);
        let minor2 = piece(MinorBuilder::prince(), PieceIdentity::Bishop, None);
        let elite = piece(EliteBuilder::chancellor(), PieceIdentity::Queen, None);

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
