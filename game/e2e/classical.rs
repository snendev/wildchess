use itertools::Itertools;

use bevy::{
    prelude::{Commands, Startup},
    utils::HashMap,
};

use bevy_geppetto::Test;

use wildchess_game::{
    components::{
        Behavior, Pattern, PieceKind, Promotable, StartPosition,
        Team::{self, Black, White},
    },
    File, GamePieces, GameplayPlugin, LocalSquare, PieceConfiguration, Rank,
};

use wildchess_ui::{EguiBoardUIPlugin, PieceIcon, PieceIcons};

fn main() {
    Test {
        label: "test classical board".to_string(),
        setup: |app| {
            app.add_plugins((GameplayPlugin, EguiBoardUIPlugin))
                .add_systems(Startup, add_classical_pieces);
        },
    }
    .run()
}

// no en passent, no "two moves forward" rule
pub fn pawn() -> Behavior {
    Behavior::default()
        .with_pattern(Pattern::forward().range(1).cannot_attack())
        .with_pattern(Pattern::diagonal_forward().range(1).can_attack())
}

// no castling
fn king() -> Behavior {
    Behavior::builder().radials().range(1).can_attack().build()
}

fn knight() -> Behavior {
    Behavior::builder()
        .knight_jumps()
        .range(1)
        .can_attack()
        .build()
}

fn bishop() -> Behavior {
    Behavior::builder().diagonals().can_attack().build()
}

fn rook() -> Behavior {
    Behavior::builder().orthogonals().can_attack().build()
}

fn queen() -> Behavior {
    Behavior::builder().radials().can_attack().build()
}

fn classical_chess_configuration() -> Vec<(PieceConfiguration, Vec<StartPosition>)> {
    let make_piece_config = |behavior: Behavior| PieceConfiguration {
        kind: PieceKind::Piece,
        behavior,
        promotable: None,
    };
    let rank_one_square = |file: File| StartPosition(LocalSquare::new(Rank::One, file));

    vec![
        // pieces
        (
            make_piece_config(rook()),
            vec![rank_one_square(File::A), rank_one_square(File::H)],
        ),
        (
            make_piece_config(knight()),
            vec![rank_one_square(File::B), rank_one_square(File::G)],
        ),
        (
            make_piece_config(bishop()),
            vec![rank_one_square(File::C), rank_one_square(File::F)],
        ),
        (make_piece_config(queen()), vec![rank_one_square(File::D)]),
        // king
        (
            PieceConfiguration {
                kind: PieceKind::King,
                behavior: king(),
                promotable: None,
            },
            vec![rank_one_square(File::E)],
        ),
        // pawns
        (
            PieceConfiguration {
                kind: PieceKind::Pawn,
                behavior: pawn(),
                promotable: Some(Promotable {
                    local_rank: Rank::Eight,
                    behaviors: vec![queen(), rook(), knight(), bishop()],
                }),
            },
            File::all()
                .map(|file| StartPosition(LocalSquare::new(Rank::Two, file)))
                .collect(),
        ),
    ]
}

fn piece_unicode(position: StartPosition, team: Team) -> char {
    match (position.0, team) {
        // king
        (
            LocalSquare {
                rank: Rank::One,
                file: File::E,
            },
            White,
        ) => '\u{2654}',
        (
            LocalSquare {
                rank: Rank::One,
                file: File::E,
            },
            Black,
        ) => '\u{265A}',
        // queen
        (
            LocalSquare {
                rank: Rank::One,
                file: File::D,
            },
            White,
        ) => '\u{2655}',
        (
            LocalSquare {
                rank: Rank::One,
                file: File::D,
            },
            Black,
        ) => '\u{265B}',
        // rook
        (
            LocalSquare {
                rank: Rank::One,
                file: File::A | File::H,
            },
            White,
        ) => '\u{2656}',
        (
            LocalSquare {
                rank: Rank::One,
                file: File::A | File::H,
            },
            Black,
        ) => '\u{265C}',
        // bishop
        (
            LocalSquare {
                rank: Rank::One,
                file: File::C | File::F,
            },
            White,
        ) => '\u{2657}',
        (
            LocalSquare {
                rank: Rank::One,
                file: File::C | File::F,
            },
            Black,
        ) => '\u{265D}',
        // knight
        (
            LocalSquare {
                rank: Rank::One,
                file: File::B | File::G,
            },
            White,
        ) => '\u{2658}',
        (
            LocalSquare {
                rank: Rank::One,
                file: File::B | File::G,
            },
            Black,
        ) => '\u{265E}',
        // pawn
        (
            LocalSquare {
                rank: Rank::Two, ..
            },
            White,
        ) => '\u{2659}',
        (
            LocalSquare {
                rank: Rank::Two, ..
            },
            Black,
        ) => '\u{265F}',
        _ => ' ',
    }
}

fn build_icons(pieces: Vec<(PieceConfiguration, Vec<StartPosition>)>) -> PieceIcons {
    PieceIcons(
        pieces
            .into_iter()
            .cartesian_product([Team::White, Team::Black])
            .flat_map(|((_, start_positions), team)| {
                start_positions.into_iter().map(move |start_position| {
                    (
                        (start_position.clone(), team),
                        PieceIcon::Character(piece_unicode(start_position, team)),
                    )
                })
            })
            .collect::<HashMap<_, _>>(),
    )
}

fn add_classical_pieces(mut commands: Commands) {
    let pieces = classical_chess_configuration();
    commands.insert_resource(GamePieces(pieces.clone()));
    commands.insert_resource(build_icons(pieces))
}
