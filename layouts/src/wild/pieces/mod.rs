use rand::{thread_rng, Rng};

use chess::{
    behavior::PatternBehavior,
    board::Rank,
    pieces::{Mutation, MutationCondition, PieceDefinition, Royal},
};

mod king;
mod pawn;
mod piece;

// Piece identity described by the starting squares
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(self) struct PieceBuilder;

// Game properties to randomly divide amongst pieces:
// ALWAYS have at least:
// 1 short range leaper pattern
// 1 diagonal rider pattern
// 1 orthogonal rider pattern

// For king (impl in king.rs)
// select randomly:
//   (the existing ones)

// For pawns (impl in pawn.rs):
// select randomly:
//   traditional
//   weak
//   torpedo
//   agile (sideways)
//   retreatable
//   berolina
//   (later on: rotatable)

pub struct WildPieceSet {
    pub elite: PieceDefinition,
    pub major: PieceDefinition,
    pub minor1: PieceDefinition,
    pub minor2: PieceDefinition,
    pub pawn: PieceDefinition,
    pub king: PieceDefinition,
}

pub fn random_pieces() -> WildPieceSet {
    let mut rng = thread_rng();
    let max_value: u32 = rng.gen_range(50..80);
    let mut current_value: u32 = 0;

    // pieces
    let major: PieceDefinition = piece(PieceBuilder::generate_piece(max_value, &mut current_value));
    let minor1 = piece(PieceBuilder::generate_piece(max_value, &mut current_value));
    let minor2 = piece(PieceBuilder::generate_piece(max_value, &mut current_value));
    let elite = piece(PieceBuilder::generate_piece(max_value, &mut current_value));

    // pawns
    let pawn_promotion_options = vec![major.clone(), minor1.clone(), minor2.clone(), elite.clone()];
    let pawn = pawn(PieceBuilder::generate_pawn(), pawn_promotion_options);
    // king
    let king = king(PieceBuilder::generate_king());

    WildPieceSet {
        elite,
        major,
        minor1,
        minor2,
        pawn,
        king,
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

fn pawn(behavior: PatternBehavior, options: Vec<PieceDefinition>) -> PieceDefinition {
    PieceDefinition {
        behaviors: behavior.into(),
        mutation: Some(Mutation {
            condition: MutationCondition::LocalRank(Rank::EIGHT),
            to_piece: options,
            ..Default::default()
        }),
        ..Default::default()
    }
}
