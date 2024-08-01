use chess::{
    behavior::RelayBehavior,
    board::{File, Rank, Square},
    pieces::{Mutation, MutationCondition, PieceDefinition, PieceIdentity, Royal},
};

use crate::{classical::pieces, PieceSpecification};

pub struct SuperRelayLayout;

impl SuperRelayLayout {
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
        behaviors: RelayBehavior::from(pieces::king()).into(),
        royal: Some(Royal),
        identity: PieceIdentity::King,
        ..Default::default()
    }
}

fn pawn() -> PieceDefinition {
    PieceDefinition {
        behaviors: RelayBehavior::from(pieces::pawn()).into(),
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
        RelayBehavior::from(pieces::rook()).into(),
        PieceIdentity::Rook,
    )
}

fn knight() -> PieceDefinition {
    PieceDefinition::new(
        RelayBehavior::from(pieces::knight()).into(),
        PieceIdentity::Knight,
    )
}

fn bishop() -> PieceDefinition {
    PieceDefinition::new(
        RelayBehavior::from(pieces::bishop()).into(),
        PieceIdentity::Bishop,
    )
}

fn queen() -> PieceDefinition {
    PieceDefinition::new(
        RelayBehavior::from(pieces::queen()).into(),
        PieceIdentity::Queen,
    )
}
