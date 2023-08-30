use itertools::Itertools;

use bevy::prelude::{App, Commands, Component, Plugin, Startup};

use chess::{
    pieces::{Behavior, King, PieceBundle, Promotable, StartPosition},
    square::{File, LocalSquare, Rank},
    team::Team,
};

use crate::pieces::{bishop, king, knight, pawn, queen, rook};

const TEAMS: [Team; 2] = [Team::White, Team::Black];

fn build_batch<'a>(
    squares: impl Iterator<Item = LocalSquare> + 'a,
    behavior: Behavior,
) -> impl Iterator<Item = PieceBundle> + 'a {
    squares
        .cartesian_product(TEAMS)
        .map(move |(square, team)| PieceBundle::new(behavior.clone(), team, StartPosition(square)))
}

#[derive(Clone, Copy, Component, Debug)]
enum PieceIdentity {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

fn rank_one_square(file: File) -> LocalSquare {
    LocalSquare::new(Rank::One, file)
}

fn setup_wild_board(mut commands: Commands) {
    // pieces
    commands.spawn_batch(
        build_batch([File::D].into_iter().map(rank_one_square), queen())
            .map(|bundle| (bundle, PieceIdentity::Queen)),
    );
    commands.spawn_batch(
        build_batch([File::A, File::H].into_iter().map(rank_one_square), rook())
            .map(|bundle| (bundle, PieceIdentity::Rook)),
    );
    commands.spawn_batch(
        build_batch(
            [File::C, File::F].into_iter().map(rank_one_square),
            bishop(),
        )
        .map(|bundle| (bundle, PieceIdentity::Bishop)),
    );
    commands.spawn_batch(
        build_batch(
            [File::B, File::G].into_iter().map(rank_one_square),
            knight(),
        )
        .map(|bundle| (bundle, PieceIdentity::Knight)),
    );

    // king
    commands.spawn_batch(
        build_batch([File::E].into_iter().map(rank_one_square), king())
            .map(|bundle| (bundle, King, PieceIdentity::King)),
    );

    // pawns
    let pawn_promotion_options = vec![queen(), rook(), bishop(), knight()];

    commands.spawn_batch(
        build_batch(
            File::all().map(|file: File| LocalSquare::new(Rank::Two, file)),
            pawn(),
        )
        .map(move |bundle| {
            let pawn_promotion = Promotable {
                ranks: vec![match bundle.team {
                    Team::White => Rank::Eight,
                    Team::Black => Rank::One,
                }],
                behaviors: pawn_promotion_options.clone(),
            };
            (bundle.promotable(pawn_promotion), PieceIdentity::Pawn)
        }),
    );
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_wild_board);
    }
}
