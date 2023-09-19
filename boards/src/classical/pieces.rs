use chess::{behavior::PatternBehavior, board::Rank, pieces::Pattern};

pub fn pawn() -> PatternBehavior {
    PatternBehavior::default()
        .with_pattern(Pattern::forward().range(1))
        .with_pattern(
            Pattern::diagonal_forward()
                .range(1)
                .only_captures_by_displacement(),
        )
        .with_pattern(Pattern::forward().range(2).only_from_local_rank(Rank::TWO))
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
