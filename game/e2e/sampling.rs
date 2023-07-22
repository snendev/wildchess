use bevy::prelude::{Commands, Startup};

use bevy_geppetto::Test;

use wildchess_game::{
    board::{
        AdvancedBuilder, EliteBuilder, InfantryBuilder, LegendaryBuilder, MajorBuilder,
        MinorBuilder, PieceBuilder, PieceBundle,
    },
    Behavior, File, GameplayPlugin, Rank, Square, Team, Vision,
};

fn main() {
    Test {
        label: "test a random piece of each piece type".to_string(),
        setup: |app| {
            app.add_plugins(GameplayPlugin)
                .add_systems(Startup, piece_sampling_board);
        },
    }
    .run();
}

fn piece_sampling_board(mut commands: Commands) {
    let infantry_behavior = InfantryBuilder.generate_wild_behavior();
    commands.spawn(PieceBundle::new(
        infantry_behavior,
        Team::White,
        'a'.try_into().unwrap(),
    ));
    let minor_behavior = MinorBuilder.generate_wild_behavior();
    commands.spawn(PieceBundle::new(
        minor_behavior,
        Team::White,
        'b'.try_into().unwrap(),
    ));
    let advanced_behavior = AdvancedBuilder.generate_wild_behavior();
    commands.spawn(PieceBundle::new(
        advanced_behavior,
        Team::White,
        'c'.try_into().unwrap(),
    ));
    let major_behavior = MajorBuilder.generate_wild_behavior();
    commands.spawn(PieceBundle::new(
        major_behavior,
        Team::White,
        'd'.try_into().unwrap(),
    ));
    let elite_behavior = EliteBuilder.generate_wild_behavior();
    commands.spawn(PieceBundle::new(
        elite_behavior,
        Team::White,
        'e'.try_into().unwrap(),
    ));
    let legendary_behavior = LegendaryBuilder.generate_wild_behavior();
    commands.spawn(PieceBundle::new(
        legendary_behavior,
        Team::White,
        'f'.try_into().unwrap(),
    ));

    // something to target
    commands.spawn(PieceBundle {
        behavior: Behavior::default(),
        team: Team::Black,
        square: Square::new(File::E, Rank::Five),
        vision: Vision::default(),
    });
}
