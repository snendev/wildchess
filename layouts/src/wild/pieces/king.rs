use rand::Rng;

use chess::{
    behavior::PatternBehavior,
    pattern::{Pattern, RSymmetry, Step},
};

use super::PieceBuilder;

impl PieceBuilder {
    pub fn generate_king() -> PatternBehavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=3) {
            // classical king
            0 => PatternBehavior::default()
                .with_pattern(Pattern::radial().leaper().captures_by_displacement()),
            // weak king
            1 => PatternBehavior::default()
                .with_pattern(Pattern::orthogonal().leaper().captures_by_displacement()),
            // silver general king
            2 => PatternBehavior::default().with_pattern(
                Pattern::new(Step::from_r(1, RSymmetry::diagonal() | RSymmetry::FORWARD))
                    .leaper()
                    .captures_by_displacement(),
            ),
            // gold general king
            _ => PatternBehavior::default().with_pattern(
                Pattern::new(Step::from_r(
                    1,
                    RSymmetry::diagonal() | RSymmetry::sideways() | RSymmetry::FORWARD,
                ))
                .leaper()
                .captures_by_displacement(),
            ),
        }
    }
}
