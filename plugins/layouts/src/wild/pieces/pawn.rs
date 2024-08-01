use rand::Rng;

use chess::{
    behavior::PatternBehavior,
    pattern::{Pattern, RSymmetry, Step},
};

use super::PieceBuilder;

impl PieceBuilder {
    pub fn generate_pawn() -> PatternBehavior {
        PawnBuilder::random_pawn()
    }
}

pub struct PawnBuilder;

impl PawnBuilder {
    pub fn random_pawn() -> PatternBehavior {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..=5) {
            0 => Self::torpedo(),
            1 => Self::historic(),
            2 => Self::retreatable(),
            3 => Self::agile(),
            4 => Self::berolina(),
            _ => Self::checker(),
        }
    }

    pub fn torpedo() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::forward().range(2))
            .with_pattern(
                Pattern::diagonal_forward()
                    .range(1)
                    .only_captures_by_displacement(),
            )
    }

    pub fn historic() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::forward().range(1))
            .with_pattern(
                Pattern::diagonal_forward()
                    .range(1)
                    .only_captures_by_displacement(),
            )
    }

    pub fn retreatable() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::new(Step::from_r(1, RSymmetry::vertical())).range(1))
            .with_pattern(
                Pattern::diagonal_forward()
                    .range(1)
                    .only_captures_by_displacement(),
            )
    }

    pub fn agile() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::forward().range(1))
            .with_pattern(Pattern::horizontal().range(1))
            .with_pattern(
                Pattern::diagonal_forward()
                    .range(1)
                    .only_captures_by_displacement(),
            )
    }

    pub fn berolina() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::diagonal_forward().range(1))
            .with_pattern(Pattern::forward().range(1).only_captures_by_displacement())
    }

    pub fn checker() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::diagonal_forward().leaper())
            .with_pattern(Pattern::diagonal_forward().range(2).captures_by_overtake())
    }
}
