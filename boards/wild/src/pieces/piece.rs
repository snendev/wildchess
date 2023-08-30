use rand::{thread_rng, Rng};

use chess::pieces::{Behavior, Pattern};

use super::PieceKind;

impl PieceKind {
    // TODO figure out a better strategy
    pub fn generate_piece(max_value: u32, current_value: &mut u32) -> Behavior {
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
    fn generate_wild_behavior() -> Behavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=3) {
            // grunt
            0 => Behavior::builder()
                .orthogonals()
                .range(3)
                .can_attack()
                .build(),
            // hound
            1 => Behavior::builder()
                .diagonal_forward()
                .range(2)
                .can_attack()
                .build()
                .join(
                    Behavior::builder()
                        .sideways()
                        .backward()
                        .cannot_attack()
                        .range(1)
                        .build(),
                ),
            // fencer
            2 => Behavior::default()
                .with_pattern(Pattern::forward().can_attack().range(2))
                .with_pattern(Pattern::forward().must_attack().jumping().range(3))
                .with_pattern(Pattern::sideways().cannot_attack().range(1)),
            // squire
            _ => Behavior::builder()
                .forward()
                .sideways()
                .range(1)
                .cannot_attack()
                .build()
                .join(
                    Behavior::builder()
                        .knight_jumps()
                        .must_attack()
                        .range(1)
                        .build(),
                ),
        }
    }
}

// "Minor" class
pub struct MinorBuilder;

impl MinorBuilder {
    fn generate_wild_behavior() -> Behavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=3) {
            // classic knight
            0 => Behavior::builder().knight_jumps().can_attack().build(),
            // classic bishop
            1 => Behavior::builder().diagonals().can_attack().build(),
            // scorpion
            2 => Behavior::default()
                .with_pattern(Pattern::backward().range(1).cannot_attack())
                .with_pattern(Pattern::diagonal_forward().jumping().range(3).must_attack())
                .join(
                    Behavior::builder()
                        .forward()
                        .sideways()
                        .range(3)
                        .cannot_attack()
                        .build(),
                ),
            // princess
            _ => Behavior::builder().radials().range(2).can_attack().build(),
        }
    }
}

// "Advanced" class
pub struct AdvancedBuilder;

impl AdvancedBuilder {
    fn generate_wild_behavior() -> Behavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=3) {
            // jester
            0 => Behavior::builder()
                .knight_jumps()
                .range(1)
                .can_attack()
                .build()
                .join(Behavior::builder().orthogonals().range(2).build()),
            // butterfly
            1 => Behavior::builder()
                .knight_jumps()
                .range(1)
                .can_attack()
                .build()
                .join(
                    Behavior::builder()
                        .radials()
                        .range(3)
                        .cannot_attack()
                        .build(),
                ),
            // dancer
            2 => Behavior::builder()
                .diagonals()
                .cannot_attack()
                .build()
                .join(Behavior::builder().orthogonals().must_attack().build()),
            // aiofe
            _ => Behavior::builder()
                .radials()
                .range(2)
                .can_attack()
                .build()
                .join(
                    Behavior::builder()
                        .knight_jumps()
                        .range(1)
                        .cannot_attack()
                        .build(),
                ),
        }
    }
}

// "Major" class
pub struct MajorBuilder;

impl MajorBuilder {
    fn generate_wild_behavior() -> Behavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=2) {
            // classic rook
            0 => Behavior::builder().orthogonals().can_attack().build(),
            // cardinal
            _ => Behavior::builder().diagonals().can_attack().build().join(
                Behavior::builder()
                    .orthogonals()
                    .range(1)
                    .cannot_attack()
                    .build(),
            ),
        }
    }
}

// "Elite" class
pub struct EliteBuilder;

impl EliteBuilder {
    fn generate_wild_behavior() -> Behavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=2) {
            // classic queen
            0 => Behavior::builder().radials().can_attack().build(),
            // chancellor
            1 => Behavior::builder().orthogonals().can_attack().build().join(
                Behavior::builder()
                    .knight_jumps()
                    .range(1)
                    .can_attack()
                    .build(),
            ),
            // panther
            2 => Behavior::builder()
                .orthogonals()
                .can_attack()
                .build()
                .with_pattern(
                    Pattern::diagonal_forward()
                        .range(3)
                        .jumping()
                        .cannot_attack(),
                ),
            // dominator
            _ => Behavior::builder()
                .radials()
                .range(3)
                .can_attack()
                .build()
                .join(
                    Behavior::builder()
                        .knight_jumps()
                        .range(1)
                        .can_attack()
                        .build(),
                ),
        }
    }
}

// "Legendary" class
pub struct LegendaryBuilder;

impl LegendaryBuilder {
    fn generate_wild_behavior() -> Behavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=2) {
            // dragon
            _ => Behavior::builder().radials().can_attack().build().join(
                Behavior::builder()
                    .knight_jumps()
                    .range(1)
                    .can_attack()
                    .build(),
            ),
            // TODO
            // pirate
            // _ => Behavior::builder()
            //     .knight_jumps()
            //     .range(2)
            //     .can_attack()
            //     .build()
            //     .with_pattern(Pattern::forward().jumping().cannot_attack())
            //     .with_pattern(Pattern::sideways().jumping().range(2).cannot_attack()),
        }
    }
}
