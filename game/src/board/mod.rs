use bevy::prelude::{Commands, Resource};

use rand::{thread_rng, Rng};

use crate::{Behavior, File, Team};

mod king;
mod pawn;
mod piece;

pub use king::{King, KingBuilder, KingBundle};
pub use pawn::{Pawn, PawnBuilder, PawnBundle};
pub use piece::{
    AdvancedBuilder, EliteBuilder, InfantryBuilder, LegendaryBuilder, MajorBuilder, MinorBuilder,
    PieceBundle, PieceIdentity,
};

#[derive(Resource)]
pub struct GamePieces(pub [(PieceIdentity, Behavior); 4]);

pub trait PieceBuilder {
    fn generate_wild_behavior(&self) -> Behavior;
}

fn generate_wild_piece(max_value: u32, current_value: &mut u32) -> Behavior {
    let mut rng = thread_rng();
    let behavior = match rng.gen_range(0u32..(max_value - *current_value)) {
        0..=9 => InfantryBuilder.generate_wild_behavior(),
        10..=19 => MinorBuilder.generate_wild_behavior(),
        20..=29 => AdvancedBuilder.generate_wild_behavior(),
        30..=39 => MajorBuilder.generate_wild_behavior(),
        40..=49 => EliteBuilder.generate_wild_behavior(),
        50..=u32::MAX => LegendaryBuilder.generate_wild_behavior(),
    };
    behavior
}

fn four_random_piece_behaviors() -> [(PieceIdentity, Behavior); 4] {
    let mut rng = thread_rng();
    let max_value: u32 = rng.gen_range(50..80);
    let mut current_value: u32 = 0;
    [
        (
            PieceIdentity::D,
            generate_wild_piece(max_value, &mut current_value),
        ),
        (
            PieceIdentity::AH,
            generate_wild_piece(max_value, &mut current_value),
        ),
        (
            PieceIdentity::BG,
            generate_wild_piece(max_value, &mut current_value),
        ),
        (
            PieceIdentity::CF,
            generate_wild_piece(max_value, &mut current_value),
        ),
    ]
}

pub fn wild_board(mut commands: Commands) {
    let king_behavior = KingBuilder.generate_wild_behavior();
    let pawn_behavior = PawnBuilder.generate_wild_behavior();
    let pieces = four_random_piece_behaviors();

    commands.insert_resource(GamePieces(pieces.clone()));

    for team in vec![Team::White, Team::Black] {
        // pawns
        for file in 0..=7 {
            commands.spawn(PawnBundle::new(
                pawn_behavior.clone(),
                team,
                file.try_into().unwrap(),
            ));
        }
        // king
        commands.spawn(KingBundle::new(
            king_behavior.clone(),
            team,
            'e'.try_into().unwrap(),
        ));

        // the pieces are placed on the board symmetrically outside->in,
        // such that the fourth behavior is only spawned once
        for (id, behavior) in pieces.iter() {
            let files = match id {
                PieceIdentity::AH => vec![File::A, File::H],
                PieceIdentity::BG => vec![File::B, File::G],
                PieceIdentity::CF => vec![File::C, File::F],
                PieceIdentity::D => vec![File::D],
            };
            for bundle in files
                .into_iter()
                .map(|file| PieceBundle::new(behavior.clone(), team, file))
            {
                commands.spawn((bundle, *id));
            }
        }
    }
}
