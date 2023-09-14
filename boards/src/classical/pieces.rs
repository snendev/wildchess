use chess::pieces::{Pattern, PatternBehavior};

pub fn pawn() -> PatternBehavior {
    PatternBehavior::default()
        .with_pattern(Pattern::forward().range(1))
        .with_pattern(
            Pattern::diagonal_forward()
                .range(1)
                .only_captures_by_displacement(),
        )
}

// no castling
pub fn king() -> PatternBehavior {
    PatternBehavior::default().with_pattern(Pattern::radial().leaper().captures_by_displacement())
}

pub fn knight() -> PatternBehavior {
    PatternBehavior::default().with_pattern(Pattern::knight().leaper().captures_by_displacement())
}

pub fn bishop() -> PatternBehavior {
    PatternBehavior::default().with_pattern(Pattern::diagonal().rider().captures_by_displacement())
}

pub fn rook() -> PatternBehavior {
    PatternBehavior::default()
        .with_pattern(Pattern::orthogonal().rider().captures_by_displacement())
}

pub fn queen() -> PatternBehavior {
    PatternBehavior::default().with_pattern(Pattern::radial().rider().captures_by_displacement())
}
