use rand::Rng;

use bevy::prelude::{Bundle, Component};

use crate::{
    behavior::Behavior,
    board::PieceBuilder,
    square::{File, Square},
    team::Team,
    Vision,
};

#[derive(Clone, Copy, Component)]
pub struct King;

#[derive(Bundle)]
pub struct KingBundle {
    pub behavior: Behavior,
    pub king: King,
    pub square: Square,
    pub team: Team,
    pub vision: Vision,
}

impl KingBundle {
    pub fn new(behavior: Behavior, team: Team, file: File) -> Self {
        KingBundle {
            behavior,
            king: King,
            square: Square::piece(file, team),
            team,
            vision: Vision::default(),
        }
    }
}

pub struct KingBuilder;

impl PieceBuilder for KingBuilder {
    fn generate_wild_behavior(&self) -> Behavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=2) {
            0 => Behavior::builder().radials().range(1).can_attack().build(),
            1 => Behavior::builder()
                .orthogonals()
                .range(1)
                .can_attack()
                .build(),
            _ => Behavior::builder()
                .forward()
                .diagonal_backward()
                .range(1)
                .can_attack()
                .build(),
        }
    }
}
