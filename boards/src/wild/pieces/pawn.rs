use rand::Rng;

use chess::pieces::{Behavior, Pattern};

use super::PieceKind;

impl PieceKind {
    pub fn generate_pawn() -> Behavior {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..=2) {
            // torpedo pawn
            0 => Behavior::default()
                .with_pattern(Pattern::forward().range(2).cannot_attack())
                .with_pattern(Pattern::diagonal_forward().range(1).must_attack()),
            // historic pawn
            1 => Behavior::default()
                .with_pattern(Pattern::forward().range(1).cannot_attack())
                .with_pattern(Pattern::diagonal_forward().range(1).must_attack()),
            // retreatable pawn
            2 => Behavior::default()
                .with_pattern(Pattern::forward().range(1).cannot_attack())
                .with_pattern(Pattern::backward().range(1).cannot_attack())
                .with_pattern(Pattern::diagonal_forward().range(1).must_attack()),
            // agile pawn
            _ => Behavior::default()
                .with_pattern(Pattern::forward().range(1).cannot_attack())
                .with_pattern(Pattern::sideways().range(1).cannot_attack())
                .with_pattern(Pattern::diagonal_forward().range(1).must_attack()),
        }
    }
}
