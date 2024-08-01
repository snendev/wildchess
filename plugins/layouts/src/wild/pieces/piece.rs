use rand::{thread_rng, Rng};

use chess::{
    behavior::PatternBehavior,
    pattern::{CapturePattern, CaptureRules, Pattern, RSymmetry, Step},
};

use super::PieceBuilder;

impl PieceBuilder {
    // TODO implement out a better strategy
    pub fn random_behavior(max_value: u32, current_value: &mut u32) -> PatternBehavior {
        let mut rng: rand::prelude::ThreadRng = thread_rng();
        let new_cost = rng.gen_range(0u32..(max_value - *current_value));
        *current_value += new_cost;
        match new_cost {
            0..=9 => InfantryBuilder::random_behavior(),
            10..=19 => MinorBuilder::random_behavior(),
            20..=29 => AdvancedBuilder::random_behavior(),
            30..=39 => MajorBuilder::random_behavior(),
            40..=49 => EliteBuilder::random_behavior(),
            50..=u32::MAX => LegendaryBuilder::random_behavior(),
        }
    }
}

pub struct InfantryBuilder;

impl InfantryBuilder {
    pub fn random_behavior() -> PatternBehavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=4) {
            0 => Self::raven(),
            1 => Self::acolyte(),
            2 => Self::hound(),
            3 => Self::grunt(),
            _ => Self::squire(),
        }
    }

    pub fn raven() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::orthogonal().range(3).captures_by_displacement())
    }

    pub fn acolyte() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::diagonal().range(3).captures_by_displacement())
    }

    pub fn hound() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(
                Pattern::diagonal_forward()
                    .range(2)
                    .captures_by_displacement(),
            )
            .with_pattern(Pattern::orthogonal().leaper())
    }

    pub fn grunt() -> PatternBehavior {
        PatternBehavior::default().with_pattern(Pattern::radial().leaper())
    }

    pub fn squire() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::new(Step::from_r(
                1,
                RSymmetry::FORWARD | RSymmetry::horizontal(),
            )))
            .with_pattern(Pattern::knight().leaper().only_captures_by_displacement())
    }
}

// "Minor" class
pub struct MinorBuilder;

impl MinorBuilder {
    pub fn random_behavior() -> PatternBehavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=6) {
            0 => Self::knight(),
            1 => Self::camel(),
            2 => Self::scorpion(),
            3 => Self::fencer(),
            4 => Self::ranger(),
            5 => Self::dancer(),
            6 => Self::prince(),
            _ => Self::sentry(),
        }
    }

    pub fn knight() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::knight().leaper().captures_by_displacement())
    }

    pub fn camel() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::camel().leaper().captures_by_displacement())
    }

    pub fn scorpion() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::backward().leaper())
            .with_pattern(Pattern::horizontal().range(3))
            .with_pattern(Pattern::forward().range(3))
            .with_pattern(
                Pattern::diagonal_forward()
                    .range(3)
                    .only_captures_by_displacement()
                    .pierces(),
            )
    }

    pub fn fencer() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(
                Pattern::forward()
                    .range(3)
                    .only_captures_by_displacement()
                    .pierces(),
            )
            .with_pattern(Pattern::forward().range(2).captures_by_displacement())
            .with_pattern(Pattern::horizontal().range(1))
    }

    pub fn ranger() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::orthogonal().range(4).captures_by_displacement())
    }

    pub fn dancer() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::diagonal())
            .with_pattern(Pattern::orthogonal().only_captures_by_displacement())
    }

    pub fn prince() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::radial().range(2).captures_by_displacement())
    }

    pub fn sentry() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::orthogonal().range(3).with_capture(CaptureRules {
                pattern: CapturePattern::CaptureInPassing,
                ..Default::default()
            }))
            .with_pattern(Pattern::orthogonal().leaper().captures_by_displacement())
    }
}

// "Advanced" class
pub struct AdvancedBuilder;

impl AdvancedBuilder {
    pub fn random_behavior() -> PatternBehavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=4) {
            0 => Self::bishop(),
            1 => Self::jester(),
            2 => Self::scoundrel(),
            3 => Self::ogre(),
            _ => Self::aiofe(),
        }
    }

    pub fn bishop() -> PatternBehavior {
        PatternBehavior::default().with_pattern(Pattern::diagonal().captures_by_displacement())
    }

    pub fn jester() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::knight().leaper().captures_by_displacement())
            .with_pattern(Pattern::orthogonal().range(2).pierces())
    }

    pub fn ogre() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::orthogonal().range(4).captures_by_displacement())
            .with_pattern(Pattern::diagonal().leaper().captures_by_displacement())
    }

    pub fn scoundrel() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::knight().leaper().captures_by_displacement())
            .with_pattern(Pattern::radial().leaper())
    }

    pub fn aiofe() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::radial().range(2).captures_by_displacement())
            .with_pattern(Pattern::knight().leaper())
    }
}

// "Major" class
pub struct MajorBuilder;

impl MajorBuilder {
    pub fn random_behavior() -> PatternBehavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=6) {
            0 => Self::rook(),
            1 => Self::cardinal(),
            2 => Self::butterfly(),
            3 => Self::lord(),
            4 => Self::ninja(),
            _ => Self::falconer(),
        }
    }

    pub fn rook() -> PatternBehavior {
        PatternBehavior::default().with_pattern(Pattern::orthogonal().captures_by_displacement())
    }

    pub fn cardinal() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::diagonal().captures_by_displacement())
            .with_pattern(Pattern::orthogonal().leaper())
    }

    pub fn butterfly() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::knight().leaper().captures_by_displacement())
            .with_pattern(
                Pattern::orthogonal()
                    .range(2)
                    .captures_by_displacement()
                    .pierces(),
            )
    }

    pub fn lord() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::knight().leaper().captures_by_displacement())
            .with_pattern(Pattern::orthogonal().range(3).captures_by_displacement())
    }

    pub fn ninja() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::diagonal().captures_by_displacement())
            .with_pattern(Pattern::knight().captures_by_displacement().leaper())
    }

    pub fn falconer() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::diagonal().captures_by_displacement())
            .with_pattern(Pattern::camel().leaper().captures_by_displacement())
    }
}

// "Elite" class
pub struct EliteBuilder;

impl EliteBuilder {
    pub fn random_behavior() -> PatternBehavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=4) {
            0 => Self::queen(),
            1 => Self::chancellor(),
            2 => Self::executioner(),
            3 => Self::panther(),
            _ => Self::dominator(),
        }
    }

    pub fn queen() -> PatternBehavior {
        PatternBehavior::default().with_pattern(Pattern::radial().captures_by_displacement())
    }

    pub fn chancellor() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::orthogonal().captures_by_displacement())
            .with_pattern(Pattern::knight().leaper().captures_by_displacement())
    }

    pub fn executioner() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::orthogonal().captures_by_displacement())
            .with_pattern(Pattern::diagonal_forward().captures_by_displacement())
    }

    pub fn panther() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::orthogonal().captures_by_displacement())
            .with_pattern(
                Pattern::diagonal_forward()
                    .range(3)
                    .captures_by_displacement()
                    .pierces(),
            )
    }

    pub fn dominator() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::radial().range(3).captures_by_displacement())
            .with_pattern(Pattern::knight().leaper().captures_by_displacement())
    }
}

// "Legendary" class
pub struct LegendaryBuilder;

impl LegendaryBuilder {
    fn random_behavior() -> PatternBehavior {
        let mut rng = rand::thread_rng();
        #[allow(clippy::match_single_binding)]
        match rng.gen_range(0..=2) {
            _ => Self::dragon(),
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

    pub fn dragon() -> PatternBehavior {
        PatternBehavior::default()
            .with_pattern(Pattern::radial().captures_by_displacement())
            .with_pattern(Pattern::knight().leaper().captures_by_displacement())
    }
}
