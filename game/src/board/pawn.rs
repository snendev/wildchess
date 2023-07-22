use bevy::prelude::{Bundle, Component};
use rand::Rng;

use crate::{
    behavior::Behavior,
    behavior::Pattern,
    board::PieceBuilder,
    square::{File, Square},
    team::Team,
    Vision,
};

#[derive(Clone, Copy, Component)]
pub struct Pawn;

#[derive(Bundle)]
pub struct PawnBundle {
    pub pawn: Pawn,
    pub behavior: Behavior,
    pub square: Square,
    pub team: Team,
    pub vision: Vision,
}

impl PawnBundle {
    pub fn new(behavior: Behavior, team: Team, file: File) -> Self {
        PawnBundle {
            pawn: Pawn,
            behavior,
            square: Square::pawn(file, team),
            team,
            vision: Vision::default(),
        }
    }
}

pub struct PawnBuilder;

impl PieceBuilder for PawnBuilder {
    fn generate_wild_behavior(&self) -> Behavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=2) {
            0 => Behavior::default()
                .with_pattern(Pattern::forward().range(2).cannot_attack())
                .with_pattern(Pattern::diagonal_forward().range(1).must_attack()),
            1 => Behavior::default()
                .with_pattern(Pattern::forward().range(1).cannot_attack())
                .with_pattern(Pattern::diagonal_forward().range(1).must_attack()),
            _ => Behavior::default()
                .with_pattern(Pattern::forward().range(1).cannot_attack())
                .with_pattern(Pattern::sideways().range(1).cannot_attack())
                .with_pattern(Pattern::diagonal_forward().range(1).must_attack()),
        }
    }
}
