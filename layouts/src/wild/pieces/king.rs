use rand::Rng;

use chess::{
    behavior::PatternBehavior,
    pattern::{Pattern, RSymmetry, Step},
};

use super::PieceBuilder;

impl PieceBuilder {
    pub fn generate_king() -> PatternBehavior {
        KingBuilder::random_king()
    }
}

pub struct KingBuilder;

impl KingBuilder {
    pub fn random_king() -> PatternBehavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=3) {
            0 => Self::classical(),
            1 => Self::frail(),
            2 => Self::silver_general(),
            _ => Self::gold_general(),
        }
    }

    pub fn classical() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::radial().leaper().captures_by_displacement())
    }

    pub fn frail() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::orthogonal().leaper().captures_by_displacement())
    }

    pub fn silver_general() -> PatternBehavior {
        PatternBehavior::default().with_pattern(
            Pattern::new(Step::from_r(1, RSymmetry::diagonal() | RSymmetry::FORWARD))
                .leaper()
                .captures_by_displacement(),
        )
    }

    pub fn gold_general() -> PatternBehavior {
        PatternBehavior::default().with_pattern(
            Pattern::new(Step::from_r(
                1,
                RSymmetry::diagonal() | RSymmetry::sideways() | RSymmetry::FORWARD,
            ))
            .leaper()
            .captures_by_displacement(),
        )
    }
}
