use chess::{
    board::{File, Rank},
    pieces::{Mutation, MutationCondition, PatternBehavior, PieceSpecification, Royal},
    team::Team,
};

use super::{pieces, ClassicalIdentity, ClassicalPiece};
use crate::utils::squares_by_team;

pub(crate) struct ClassicalLayout;

impl ClassicalLayout {
    pub(crate) fn pieces() -> Vec<PieceSpecification<ClassicalIdentity>> {
        squares_by_team(0, [File::A, File::H].into_iter())
            .map(|(team, square)| PieceSpecification::new(rook(), team, square.into()))
            .chain(
                squares_by_team(0, [File::B, File::G].into_iter())
                    .map(|(team, square)| PieceSpecification::new(knight(), team, square.into())),
            )
            .chain(
                squares_by_team(0, [File::C, File::F].into_iter())
                    .map(|(team, square)| PieceSpecification::new(bishop(), team, square.into())),
            )
            .chain(
                squares_by_team(0, std::iter::once(File::D))
                    .map(|(team, square)| PieceSpecification::new(queen(), team, square.into())),
            )
            .chain(
                squares_by_team(0, std::iter::once(File::E))
                    .map(|(team, square)| PieceSpecification::new(king(), team, square.into())),
            )
            .chain(
                squares_by_team(1, (0..8).map(|file| File::from(file))).map(|(team, square)| {
                    PieceSpecification::new(
                        pawn(match team {
                            Team::White => Rank::EIGHT,
                            Team::Black => Rank::ONE,
                        }),
                        team,
                        square.into(),
                    )
                }),
            )
            .collect()
    }
}

fn king() -> ClassicalPiece {
    ClassicalPiece {
        behavior: pieces::king(),
        royal: Some(Royal),
        extra: ClassicalIdentity::King,
        ..Default::default()
    }
}

fn pawn(promotion_rank: Rank) -> ClassicalPiece {
    ClassicalPiece {
        behavior: pieces::pawn(),
        mutation: Some(Mutation::<ClassicalIdentity> {
            condition: MutationCondition::Rank(promotion_rank),
            options: vec![queen(), rook(), bishop(), knight()],
        }),
        extra: ClassicalIdentity::Pawn,
        ..Default::default()
    }
}

fn piece(behavior: PatternBehavior, identity: ClassicalIdentity) -> ClassicalPiece {
    ClassicalPiece {
        behavior,
        extra: identity,
        ..Default::default()
    }
}

fn rook() -> ClassicalPiece {
    piece(pieces::rook(), ClassicalIdentity::Rook)
}

fn knight() -> ClassicalPiece {
    piece(pieces::knight(), ClassicalIdentity::Knight)
}

fn bishop() -> ClassicalPiece {
    piece(pieces::bishop(), ClassicalIdentity::Bishop)
}

fn queen() -> ClassicalPiece {
    piece(pieces::queen(), ClassicalIdentity::Queen)
}
