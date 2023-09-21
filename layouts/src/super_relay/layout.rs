use bevy::prelude::Commands;

use chess::{
    behavior::RelayBehavior,
    board::{File, Rank},
    pieces::{Mutation, MutationCondition, PieceDefinition, PieceSpecification, Royal},
    team::Team,
};

use crate::{
    classical::{pieces, ClassicalIdentity},
    utils::squares_by_team,
};

pub struct SuperRelayLayout;

impl SuperRelayLayout {
    pub fn spawn_pieces(commands: &mut Commands) {
        for (piece, identity) in squares_by_team(0, [File::A, File::H].into_iter())
            .map(|(team, square)| {
                (
                    PieceSpecification::new(rook(), team, square.into()),
                    ClassicalIdentity::Rook,
                )
            })
            .chain(
                squares_by_team(0, [File::B, File::G].into_iter()).map(|(team, square)| {
                    (
                        PieceSpecification::new(knight(), team, square.into()),
                        ClassicalIdentity::Knight,
                    )
                }),
            )
            .chain(
                squares_by_team(0, [File::C, File::F].into_iter()).map(|(team, square)| {
                    (
                        PieceSpecification::new(bishop(), team, square.into()),
                        ClassicalIdentity::Bishop,
                    )
                }),
            )
            .chain(
                squares_by_team(0, std::iter::once(File::D)).map(|(team, square)| {
                    (
                        PieceSpecification::new(queen(), team, square.into()),
                        ClassicalIdentity::Queen,
                    )
                }),
            )
            .chain(
                squares_by_team(0, std::iter::once(File::E)).map(|(team, square)| {
                    (
                        PieceSpecification::new(king(), team, square.into()),
                        ClassicalIdentity::King,
                    )
                }),
            )
            .chain(
                squares_by_team(1, (0..8).map(|file| File::from(file))).map(|(team, square)| {
                    (
                        PieceSpecification::new(
                            pawn(match team {
                                Team::White => Rank::EIGHT,
                                Team::Black => Rank::ONE,
                            }),
                            team,
                            square.into(),
                        ),
                        ClassicalIdentity::Pawn,
                    )
                }),
            )
        {
            let entity = piece.spawn(commands);
            commands.entity(entity).insert(identity);
        }
    }
}

fn king() -> PieceDefinition {
    PieceDefinition {
        behaviors: RelayBehavior::from(pieces::king()).into(),
        royal: Some(Royal),
        ..Default::default()
    }
}

fn pawn(promotion_rank: Rank) -> PieceDefinition {
    PieceDefinition {
        behaviors: RelayBehavior::from(pieces::pawn()).into(),
        mutation: Some(Mutation {
            condition: MutationCondition::Rank(promotion_rank),
            to_piece: vec![queen(), rook(), bishop(), knight()],
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn rook() -> PieceDefinition {
    PieceDefinition::new(RelayBehavior::from(pieces::rook()).into())
}

fn knight() -> PieceDefinition {
    PieceDefinition::new(RelayBehavior::from(pieces::knight()).into())
}

fn bishop() -> PieceDefinition {
    PieceDefinition::new(RelayBehavior::from(pieces::bishop()).into())
}

fn queen() -> PieceDefinition {
    PieceDefinition::new(RelayBehavior::from(pieces::queen()).into())
}
