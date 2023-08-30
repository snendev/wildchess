use itertools::Itertools;

use bevy::prelude::{App, Commands, Plugin, Startup};

use chess::{
    pieces::{Behavior, King, PieceBundle, Promotable, StartPosition},
    square::{File, LocalSquare, Rank},
    team::Team,
};

mod wild_pieces;

const TEAMS: [Team; 2] = [Team::White, Team::Black];

fn build_batch<'a>(
    squares: impl Iterator<Item = LocalSquare> + 'a,
    behavior: Behavior,
) -> impl Iterator<Item = PieceBundle> + 'a {
    squares
        .cartesian_product(TEAMS)
        .map(move |(square, team)| PieceBundle::new(behavior.clone(), team, StartPosition(square)))
}

fn setup_wild_board(mut commands: Commands) {
    let piece_set = wild_pieces::random_pieces();

    let rank_one_square = |file: File| LocalSquare::new(Rank::One, file);

    // pieces
    commands.spawn_batch(build_batch(
        [File::A, File::H].into_iter().map(rank_one_square),
        piece_set.pieces.0.clone(),
    ));
    commands.spawn_batch(build_batch(
        [File::B, File::G].into_iter().map(rank_one_square),
        piece_set.pieces.1.clone(),
    ));
    commands.spawn_batch(build_batch(
        [File::C, File::F].into_iter().map(rank_one_square),
        piece_set.pieces.2.clone(),
    ));
    commands.spawn_batch(build_batch(
        [File::D].into_iter().map(rank_one_square),
        piece_set.pieces.3.clone(),
    ));

    // king
    commands.spawn_batch(
        build_batch(
            [File::E].into_iter().map(rank_one_square),
            piece_set.king.clone(),
        )
        .map(|bundle| (bundle, King)),
    );

    // pawns
    let pawn_promotion_options = vec![
        piece_set.pieces.0.clone(),
        piece_set.pieces.1.clone(),
        piece_set.pieces.2.clone(),
        piece_set.pieces.3.clone(),
    ];

    commands.spawn_batch(
        build_batch(
            File::all().map(|file: File| LocalSquare::new(Rank::Two, file)),
            piece_set.pawn.clone(),
        )
        .map(move |bundle| {
            let pawn_promotion = Promotable {
                ranks: vec![match bundle.team {
                    Team::White => Rank::Eight,
                    Team::Black => Rank::One,
                }],
                behaviors: pawn_promotion_options.clone(),
            };
            bundle.promotable(pawn_promotion)
        }),
    );
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_wild_board);
    }
}
