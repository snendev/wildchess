use chess::{
    behavior::{CastlingBehavior, CastlingTarget, EnPassantBehavior, PieceBehaviors},
    board::{File, Rank, Square},
    pieces::{Mutation, MutationCondition, PieceDefinition, PieceIdentity, Royal},
};

use super::pieces;

use crate::PieceSpecification;

pub struct ClassicalLayout;

impl ClassicalLayout {
    pub fn pieces() -> Vec<PieceSpecification> {
        [File::A, File::H]
            .into_iter()
            .map(|file| PieceSpecification::new(rook(), Square::new(file, Rank::ONE)))
            .chain(
                [File::B, File::G]
                    .into_iter()
                    .map(|file| PieceSpecification::new(knight(), Square::new(file, Rank::ONE))),
            )
            .chain(
                [File::C, File::F]
                    .into_iter()
                    .map(|file| PieceSpecification::new(bishop(), Square::new(file, Rank::ONE))),
            )
            .chain(
                std::iter::once(File::D)
                    .map(|file| PieceSpecification::new(queen(), Square::new(file, Rank::ONE))),
            )
            .chain(
                std::iter::once(File::E)
                    .map(|file| PieceSpecification::new(king(), Square::new(file, Rank::ONE))),
            )
            .chain(
                (0..8)
                    .map(File::from)
                    .map(|file| PieceSpecification::new(pawn(), Square::new(file, Rank::TWO))),
            )
            .collect()
    }
}

fn king() -> PieceDefinition {
    PieceDefinition {
        behaviors: PieceBehaviors {
            pattern: Some(pieces::king()),
            castling: Some(CastlingBehavior),
            ..Default::default()
        },
        royal: Some(Royal),
        identity: PieceIdentity::King,
        ..Default::default()
    }
}

fn pawn() -> PieceDefinition {
    PieceDefinition {
        behaviors: PieceBehaviors {
            pattern: pieces::pawn().into(),
            en_passant: Some(EnPassantBehavior),
            ..Default::default()
        },
        mutation: Some(Mutation {
            condition: MutationCondition::LocalRank(Rank::EIGHT),
            to_piece: vec![queen(), rook(), bishop(), knight()],
            ..Default::default()
        }),
        identity: PieceIdentity::Pawn,
        ..Default::default()
    }
}

fn rook() -> PieceDefinition {
    PieceDefinition::new(
        PieceBehaviors {
            pattern: Some(pieces::rook()),
            castling_target: Some(CastlingTarget),
            ..Default::default()
        },
        PieceIdentity::Rook,
    )
}

fn knight() -> PieceDefinition {
    PieceDefinition::new(pieces::knight().into(), PieceIdentity::Knight)
}

fn bishop() -> PieceDefinition {
    PieceDefinition::new(pieces::bishop().into(), PieceIdentity::Bishop)
}

fn queen() -> PieceDefinition {
    PieceDefinition::new(pieces::queen().into(), PieceIdentity::Queen)
}
