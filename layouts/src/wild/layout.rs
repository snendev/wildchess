use rand::{thread_rng, Rng};

use bevy::prelude::Commands;

use chess::{
    behavior::PatternBehavior,
    board::{File, Rank},
    pieces::{Mutation, MutationCondition, PieceDefinition, PieceSpecification, Royal},
    team::Team,
};

use crate::{utils::squares_by_team, wild::PieceBuilder};

pub struct WildLayout;

impl WildLayout {
    pub fn spawn_pieces(commands: &mut Commands) {
        let piece_set = random_pieces();

        let pawn_promotion_options = vec![
            PieceDefinition {
                behaviors: piece_set.pieces.0.clone().into(),
                ..Default::default()
            },
            PieceDefinition {
                behaviors: piece_set.pieces.1.clone().into(),
                ..Default::default()
            },
            PieceDefinition {
                behaviors: piece_set.pieces.2.clone().into(),
                ..Default::default()
            },
            PieceDefinition {
                behaviors: piece_set.pieces.3.clone().into(),
                ..Default::default()
            },
        ];

        for piece in squares_by_team(0, [File::A, File::H].into_iter())
            .map(|(team, square)| {
                PieceSpecification::new(piece(piece_set.pieces.0.clone()), team, square.into())
            })
            .chain(
                squares_by_team(0, [File::B, File::G].into_iter()).map(|(team, square)| {
                    PieceSpecification::new(piece(piece_set.pieces.1.clone()), team, square.into())
                }),
            )
            .chain(
                squares_by_team(0, [File::C, File::F].into_iter()).map(|(team, square)| {
                    PieceSpecification::new(piece(piece_set.pieces.2.clone()), team, square.into())
                }),
            )
            .chain(
                squares_by_team(0, std::iter::once(File::D)).map(|(team, square)| {
                    PieceSpecification::new(piece(piece_set.pieces.3.clone()), team, square.into())
                }),
            )
            .chain(
                squares_by_team(0, std::iter::once(File::E)).map(|(team, square)| {
                    PieceSpecification::new(king(piece_set.king.clone()), team, square.into())
                }),
            )
            .chain(
                squares_by_team(1, (0..8).map(|file| File::from(file))).map(|(team, square)| {
                    PieceSpecification::new(
                        pawn(
                            piece_set.pawn.clone(),
                            pawn_promotion(
                                match team {
                                    Team::White => Rank::EIGHT,
                                    Team::Black => Rank::ONE,
                                },
                                pawn_promotion_options.clone(),
                            ),
                        ),
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

fn piece(behavior: PatternBehavior) -> PieceDefinition {
    PieceDefinition {
        behaviors: behavior.into(),
        ..Default::default()
    }
}

fn king(behavior: PatternBehavior) -> PieceDefinition {
    PieceDefinition {
        behaviors: behavior.into(),
        royal: Some(Royal),
        ..Default::default()
    }
}

fn pawn(behavior: PatternBehavior, mutation: Mutation) -> PieceDefinition {
    PieceDefinition {
        behaviors: behavior.into(),
        mutation: Some(mutation),
        ..Default::default()
    }
}

fn pawn_promotion(rank: Rank, options: Vec<PieceDefinition>) -> Mutation {
    Mutation {
        condition: MutationCondition::Rank(rank),
        to_piece: options,
        ..Default::default()
    }
}

struct WildPieceSet {
    pub pieces: (
        PatternBehavior,
        PatternBehavior,
        PatternBehavior,
        PatternBehavior,
    ),
    pub pawn: PatternBehavior,
    pub king: PatternBehavior,
}

fn random_pieces() -> WildPieceSet {
    let mut rng = thread_rng();
    let max_value: u32 = rng.gen_range(50..80);
    let mut current_value: u32 = 0;

    // pieces
    let ah = PieceBuilder::generate_piece(max_value, &mut current_value);
    let bg = PieceBuilder::generate_piece(max_value, &mut current_value);
    let cf = PieceBuilder::generate_piece(max_value, &mut current_value);
    let d = PieceBuilder::generate_piece(max_value, &mut current_value);
    // pawns
    let pawn = PieceBuilder::generate_pawn();
    // king
    let king = PieceBuilder::generate_king();

    WildPieceSet {
        pieces: (ah, bg, cf, d),
        pawn,
        king,
    }
}
