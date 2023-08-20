use rand::Rng;

use crate::components::{Behavior, Pattern, PieceConfiguration, PieceKind, Promotable};

impl PieceKind {
    pub fn generate_pawn(promotable: Promotable) -> PieceConfiguration {
        let mut rng = rand::thread_rng();
        PieceConfiguration {
            kind: Self::Pawn,
            behavior: match rng.gen_range(0..=2) {
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
            },
            promotable: Some(promotable),
        }
    }
}
