use rand::Rng;

use chess::pieces::{Pattern, PatternBehavior, RSymmetry, Step};

use super::PieceKind;

impl PieceKind {
    pub fn generate_pawn() -> PatternBehavior {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..=4) {
            // torpedo pawn
            0 => PatternBehavior::default()
                .with_pattern(Pattern::forward().range(2))
                .with_pattern(
                    Pattern::diagonal_forward()
                        .range(1)
                        .only_captures_by_displacement(),
                ),
            // historic pawn
            1 => PatternBehavior::default()
                .with_pattern(Pattern::forward().range(1))
                .with_pattern(
                    Pattern::diagonal_forward()
                        .range(1)
                        .only_captures_by_displacement(),
                ),
            // retreatable pawn
            2 => PatternBehavior::default()
                .with_pattern(Pattern::new(Step::from_r(1, RSymmetry::vertical())).range(1))
                .with_pattern(
                    Pattern::diagonal_forward()
                        .range(1)
                        .only_captures_by_displacement(),
                ),
            // agile pawn
            3 => PatternBehavior::default()
                .with_pattern(
                    Pattern::new(Step::from_r(
                        1,
                        RSymmetry::FORWARD | RSymmetry::horizontal(),
                    ))
                    .range(1),
                )
                .with_pattern(
                    Pattern::diagonal_forward()
                        .range(1)
                        .only_captures_by_displacement(),
                ),
            // Berolina pawn
            _ => PatternBehavior::default()
                .with_pattern(Pattern::diagonal_forward().range(1))
                .with_pattern(Pattern::forward().range(1).only_captures_by_displacement()),
        }
    }
}
