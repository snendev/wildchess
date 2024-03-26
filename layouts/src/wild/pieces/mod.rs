use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};

use chess::{
    behavior::{CastlingBehavior, CastlingTarget, PatternBehavior, PieceBehaviors},
    board::Rank,
    pieces::{Mutation, MutationCondition, PieceDefinition, Royal},
};

mod king;
mod pawn;
mod piece;

// Piece identity described by the starting squares
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct PieceBuilder;

enum PowerProperty {
    ShortRange,
    LongRange,
    Orthogonal,
    Diagonal,
    Rider,
    CornerJump,
}

const MAX_SHORT_RANGE: usize = 2;
const MAX_LONG_RANGE: usize = 3;
const MAX_ORTHOGONALS: usize = 2;
const MAX_DIAGONALS: usize = 2;
const MAX_RIDER: usize = 3;
const MAX_CORNER_JUMP: usize = 1;

enum RangePower {
    Low,
    High,
}

impl Distribution<RangePower> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RangePower {
        match rng.gen_range(0..1) {
            0 => RangePower::Low,
            _ => RangePower::High,
        }
    }
}

enum DirectionalPower {
    AllDirections,
    NoForward,
    NoBackward,
}

impl Distribution<DirectionalPower> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DirectionalPower {
        match rng.gen_range(0..1) {
            0 => DirectionalPower::AllDirections,
            1 => DirectionalPower::NoForward,
            _ => DirectionalPower::NoBackward,
        }
    }
}

fn idk() {
    // track how many pieces have each important property
    let mut total_short_range = 0;
    let mut total_long_range = 0;
    let mut total_orthogonal_range = 0;
    let mut total_diagonal_range = 0;
    let mut total_leaper = 0;
    let mut total_corner_jump = 0;
    let mut total_backward = 0;

    let mut elite = PatternBehavior::default();

    while true {
        // pick a random property to add to the unit
        // TODO
        // roll a random set of Power values
        let (range, direction) = rand::random::<(RangePower, DirectionalPower)>();
        // return a Pattern that matches the property but is scaled to the range/direction
        break;
    }
}

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
    let major: PieceDefinition = piece(
        PieceBuilder::generate_piece(max_value, &mut current_value),
        Some(CastlingTarget),
    );
    let minor1 = piece(
        PieceBuilder::generate_piece(max_value, &mut current_value),
        None,
    );
    let minor2 = piece(
        PieceBuilder::generate_piece(max_value, &mut current_value),
        None,
    );
    let elite = piece(
        PieceBuilder::generate_piece(max_value, &mut current_value),
        None,
    );

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

fn piece(behavior: PatternBehavior, castling_target: Option<CastlingTarget>) -> PieceDefinition {
    PieceDefinition {
        behaviors: PieceBehaviors {
            pattern: Some(behavior),
            castling_target,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn king(behavior: PatternBehavior) -> PieceDefinition {
    PieceDefinition {
        behaviors: PieceBehaviors {
            pattern: Some(behavior),
            castling: Some(CastlingBehavior),
            ..Default::default()
        },
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
