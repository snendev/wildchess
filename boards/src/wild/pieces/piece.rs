use rand::{thread_rng, Rng};

use chess::{
    behavior::PatternBehavior,
    pieces::{Pattern, RSymmetry, ScanMode, Step},
};

use super::PieceKind;

impl PieceKind {
    // TODO implement out a better strategy
    pub fn generate_piece(max_value: u32, current_value: &mut u32) -> PatternBehavior {
        let mut rng = thread_rng();
        let new_cost = rng.gen_range(0u32..(max_value - *current_value));
        *current_value += new_cost;
        match new_cost {
            0..=9 => InfantryBuilder::generate_wild_behavior(),
            10..=19 => MinorBuilder::generate_wild_behavior(),
            20..=29 => AdvancedBuilder::generate_wild_behavior(),
            30..=39 => MajorBuilder::generate_wild_behavior(),
            40..=49 => EliteBuilder::generate_wild_behavior(),
            50..=u32::MAX => LegendaryBuilder::generate_wild_behavior(),
        }
    }
}

struct InfantryBuilder;

impl InfantryBuilder {
    fn generate_wild_behavior() -> PatternBehavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=3) {
            // grunt
            0 => PatternBehavior::default()
                .with_pattern(Pattern::orthogonal().range(3).captures_by_displacement()),
            // hound
            1 => PatternBehavior::default()
                .with_pattern(
                    Pattern::diagonal_forward()
                        .range(2)
                        .captures_by_displacement(),
                )
                .with_pattern(
                    Pattern::new(Step::from_r(
                        1,
                        RSymmetry::BACKWARD | RSymmetry::horizontal(),
                    ))
                    .range(1),
                ),
            // fencer
            2 => PatternBehavior::default()
                .with_pattern(Pattern::forward().range(2).captures_by_displacement())
                .with_pattern(
                    Pattern::forward()
                        .range(3)
                        .scan_mode(ScanMode::Pierce)
                        .only_captures_by_displacement(),
                )
                .with_pattern(Pattern::horizontal().range(1)),
            // squire
            _ => PatternBehavior::default()
                .with_pattern(Pattern::new(Step::from_r(
                    1,
                    RSymmetry::FORWARD | RSymmetry::horizontal(),
                )))
                .with_pattern(Pattern::knight().leaper().only_captures_by_displacement()),
        }
    }
}

// "Minor" class
pub struct MinorBuilder;

impl MinorBuilder {
    fn generate_wild_behavior() -> PatternBehavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=3) {
            // classic knight
            0 => PatternBehavior::default()
                .with_pattern(Pattern::knight().captures_by_displacement()),
            // clPatternBehaviorshop
            1 => PatternBehavior::default()
                .with_pattern(Pattern::diagonal().captures_by_displacement()),
            // scPatternBehavior
            2 => PatternBehavior::default()
                .with_pattern(Pattern::backward().leaper())
                .with_pattern(
                    Pattern::diagonal_forward()
                        .scan_mode(ScanMode::Pierce)
                        .range(3)
                        .only_captures_by_displacement(),
                )
                .with_pattern(
                    Pattern::new(Step::from_r(
                        1,
                        RSymmetry::FORWARD | RSymmetry::horizontal(),
                    ))
                    .range(3),
                ),
            // prPatternBehavior
            _ => PatternBehavior::default()
                .with_pattern(Pattern::radial().range(2).captures_by_displacement()),
        }
    }
}

// "Advanced" class
pub struct AdvancedBuilder;

impl AdvancedBuilder {
    fn generate_wild_behavior() -> PatternBehavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=3) {
            // jester
            0 => PatternBehavior::default()
                .with_pattern(Pattern::knight().leaper().captures_by_displacement())
                .with_pattern(Pattern::orthogonal().range(2)),
            // butterfly
            1 => PatternBehavior::default()
                .with_pattern(Pattern::knight().leaper().captures_by_displacement())
                .with_pattern(Pattern::radial().range(3)),
            // dancer
            2 => PatternBehavior::default()
                .with_pattern(Pattern::diagonal())
                .with_pattern(Pattern::orthogonal().only_captures_by_displacement()),
            // aiofe
            _ => PatternBehavior::default()
                .with_pattern(Pattern::radial().range(2).captures_by_displacement())
                .with_pattern(Pattern::knight().leaper()),
        }
    }
}

// "Major" class
pub struct MajorBuilder;

impl MajorBuilder {
    fn generate_wild_behavior() -> PatternBehavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=2) {
            // classic rook
            0 => PatternBehavior::default()
                .with_pattern(Pattern::orthogonal().captures_by_displacement()),
            // cardinal
            _ => PatternBehavior::default()
                .with_pattern(Pattern::diagonal().captures_by_displacement())
                .with_pattern(Pattern::orthogonal().leaper()),
        }
    }
}

// "Elite" class
pub struct EliteBuilder;

impl EliteBuilder {
    fn generate_wild_behavior() -> PatternBehavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=2) {
            // classic queen
            0 => PatternBehavior::default()
                .with_pattern(Pattern::radial().captures_by_displacement()),
            // chancellor
            1 => PatternBehavior::default()
                .with_pattern(Pattern::orthogonal().captures_by_displacement())
                .with_pattern(Pattern::knight().leaper().captures_by_displacement()),
            // panther
            2 => PatternBehavior::default()
                .with_pattern(Pattern::orthogonal().captures_by_displacement())
                .with_pattern(
                    Pattern::diagonal_forward()
                        .range(3)
                        .scan_mode(ScanMode::Pierce),
                ),
            // dominator
            _ => PatternBehavior::default()
                .with_pattern(Pattern::radial().range(3).captures_by_displacement())
                .with_pattern(Pattern::knight().leaper().captures_by_displacement()),
        }
    }
}

// "Legendary" class
pub struct LegendaryBuilder;

impl LegendaryBuilder {
    fn generate_wild_behavior() -> PatternBehavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=2) {
            // dragon
            _ => PatternBehavior::default()
                .with_pattern(Pattern::radial().captures_by_displacement())
                .with_pattern(Pattern::knight().leaper().captures_by_displacement()),
            // TODO
            // pirate
            // _ => PatternBehavior::builder()
            //     .knight_jumps()
            //     .range(2)
            //     .can_attack()
            //     .build()
            //     .with_pattern(Pattern::forward().jumping().cannot_attack())
            //     .with_pattern(Pattern::sideways().jumping().range(2).cannot_attack()),
        }
    }
}
