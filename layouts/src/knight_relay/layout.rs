use bevy::prelude::Commands;

use chess::{
    behavior::{PieceBehaviors, RelayBehavior},
    board::{File, Rank},
    pieces::{
        Mutation, MutationCondition, PieceDefinition, PieceIdentity, PieceSpecification, Royal,
    },
    team::Team,
};

use crate::{classical::pieces, utils::squares_by_team};

pub struct KnightRelayLayout;

impl KnightRelayLayout {
    pub fn spawn_pieces(commands: &mut Commands) {
        for piece in squares_by_team(0, [File::A, File::H].into_iter())
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
        {
            piece.spawn(commands);
        }
    }
}

fn king() -> PieceDefinition {
    PieceDefinition {
        behaviors: pieces::king().into(),
        royal: Some(Royal),
        identity: PieceIdentity::King,
        ..Default::default()
    }
}

fn pawn(promotion_rank: Rank) -> PieceDefinition {
    PieceDefinition {
        behaviors: pieces::pawn().into(),
        mutation: Some(Mutation {
            condition: MutationCondition::Rank(promotion_rank),
            to_piece: vec![queen(), rook(), bishop(), knight()],
            ..Default::default()
        }),
        identity: PieceIdentity::Pawn,
        ..Default::default()
    }
}

fn rook() -> PieceDefinition {
    PieceDefinition::new(pieces::rook().into(), PieceIdentity::Rook)
}

fn knight() -> PieceDefinition {
    let pattern_behavior = pieces::knight();
    PieceDefinition::new(
        PieceBehaviors {
            relay: Some(RelayBehavior::from(pattern_behavior.clone())),
            pattern: Some(pattern_behavior),
            ..Default::default()
        },
        PieceIdentity::Knight,
    )
}

fn bishop() -> PieceDefinition {
    PieceDefinition::new(pieces::bishop().into(), PieceIdentity::Bishop)
}

fn queen() -> PieceDefinition {
    PieceDefinition::new(pieces::queen().into(), PieceIdentity::Queen)
}
