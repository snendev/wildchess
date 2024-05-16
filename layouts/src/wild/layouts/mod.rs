use chess::{
    behavior::{CastlingBehavior, CastlingTarget, PatternBehavior, PieceBehaviors},
    board::{File, Rank, Square},
    pieces::{Mutation, MutationCondition, PieceDefinition, PieceIdentity, Royal},
};

mod classic;
pub use classic::ClassicWildLayout;
mod featured;
pub use featured::FeaturedWildLayout;
mod random;
pub use random::RandomWildLayout;

use crate::PieceSpecification;

pub struct WildPieceSet {
    // "queen", the d file ranks 1/8
    pub elite: PieceDefinition,
    // "rook", the ah files ranks 1/8
    pub major: PieceDefinition,
    // "knight", the bg files ranks 1/8
    pub minor1: PieceDefinition,
    // "bishop", the cf files ranks 1/8
    pub minor2: PieceDefinition,
    // "pawn", all files rank 2/7
    pub pawn: PieceDefinition,
    // "king", royal on e1/e8
    pub king: PieceDefinition,
}

impl WildPieceSet {
    fn build_layout(self) -> Vec<PieceSpecification> {
        [File::A, File::H]
            .into_iter()
            .map(move |file| {
                PieceSpecification::new(self.major.clone(), Square::new(file, Rank::ONE))
            })
            .chain([File::B, File::G].into_iter().map(move |file| {
                PieceSpecification::new(self.minor1.clone(), Square::new(file, Rank::ONE))
            }))
            .chain([File::C, File::F].into_iter().map(move |file| {
                PieceSpecification::new(self.minor2.clone(), Square::new(file, Rank::ONE))
            }))
            .chain(std::iter::once(File::D).map(move |file| {
                PieceSpecification::new(self.elite.clone(), Square::new(file, Rank::ONE))
            }))
            .chain(std::iter::once(File::E).map(move |file| {
                PieceSpecification::new(self.king.clone(), Square::new(file, Rank::ONE))
            }))
            .chain((0..8).map(File::from).map(move |file| {
                PieceSpecification::new(self.pawn.clone(), Square::new(file, Rank::TWO))
            }))
            .collect()
    }
}

fn king(behavior: PatternBehavior) -> PieceDefinition {
    PieceDefinition {
        behaviors: PieceBehaviors {
            pattern: Some(behavior),
            castling: Some(CastlingBehavior),
            ..Default::default()
        },
        identity: PieceIdentity::King,
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
        identity: PieceIdentity::Pawn,
        ..Default::default()
    }
}

fn piece(
    behavior: PatternBehavior,
    identity: PieceIdentity,
    castling_target: Option<CastlingTarget>,
) -> PieceDefinition {
    PieceDefinition {
        behaviors: PieceBehaviors {
            pattern: Some(behavior),
            castling_target,
            ..Default::default()
        },
        identity,
        ..Default::default()
    }
}
