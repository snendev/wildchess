use bevy::prelude::{Added, Changed, Commands, Entity, Or, PostUpdate, Query, Startup};

use bevy_geppetto::Test;

use wildchess_game::{
    components::{
        Behavior, Board, BoardPieces, Pattern, PieceConfiguration, PieceKind, PlayerBundle,
        Promotable, StartPosition,
        Team::{self, Black, White},
        Turn,
    },
    File, GameplayPlugin, LocalSquare, Rank,
};

use wildchess_ui::{EguiBoardUIPlugin, PieceIcon};

fn main() {
    Test {
        label: "test classical board".to_string(),
        setup: |app| {
            app.add_plugins((GameplayPlugin, EguiBoardUIPlugin))
                .add_systems(Startup, add_classical_pieces)
                .add_systems(PostUpdate, override_icons);
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

fn classical_chess_configuration() -> Vec<(PieceConfiguration, Vec<StartPosition>, PieceIdentity)> {
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
            PieceIdentity::Rook,
        ),
        (
            make_piece_config(knight()),
            vec![rank_one_square(File::B), rank_one_square(File::G)],
            PieceIdentity::Knight,
        ),
        (
            make_piece_config(bishop()),
            vec![rank_one_square(File::C), rank_one_square(File::F)],
            PieceIdentity::Bishop,
        ),
        (
            make_piece_config(queen()),
            vec![rank_one_square(File::D)],
            PieceIdentity::Queen,
        ),
        // king
        (
            PieceConfiguration {
                kind: PieceKind::King,
                behavior: king(),
                promotable: None,
            },
            vec![rank_one_square(File::E)],
            PieceIdentity::King,
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
            PieceIdentity::Pawn,
        ),
    ]
}

#[derive(Clone, Debug)]
enum PieceIdentity {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

fn add_classical_pieces(mut commands: Commands) {
    let pieces = classical_chess_configuration();
    commands.spawn(Board::from_pieces(BoardPieces(
        pieces
            .iter()
            .map(|(config, start_position, _)| (config.clone(), start_position.clone()))
            .collect(),
    )));
    commands.spawn((PlayerBundle::new(Team::White), Turn));
    commands.spawn(PlayerBundle::new(Team::Black));
}

fn piece_unicode(piece: PieceIdentity, team: Team) -> char {
    match (piece, team) {
        // king
        (PieceIdentity::King, White) => '\u{2654}',
        (PieceIdentity::King, Black) => '\u{265A}',
        // queen
        (PieceIdentity::Queen, White) => '\u{2655}',
        (PieceIdentity::Queen, Black) => '\u{265B}',
        // rook
        (PieceIdentity::Rook, White) => '\u{2656}',
        (PieceIdentity::Rook, Black) => '\u{265C}',
        // bishop
        (PieceIdentity::Bishop, White) => '\u{2657}',
        (PieceIdentity::Bishop, Black) => '\u{265D}',
        // knight
        (PieceIdentity::Knight, White) => '\u{2658}',
        (PieceIdentity::Knight, Black) => '\u{265E}',
        // pawn
        (PieceIdentity::Pawn, White) => '\u{2659}',
        (PieceIdentity::Pawn, Black) => '\u{265F}',
    }
}

fn override_icons(
    mut commands: Commands,
    mut query: Query<
        (Entity, &Behavior, &Team, Option<&mut PieceIcon>),
        Or<(Changed<Behavior>, Added<PieceIcon>)>,
    >,
) {
    for (entity, behavior, team, icon) in query.iter_mut() {
        let identity = if *behavior == king() {
            PieceIdentity::King
        } else if *behavior == queen() {
            PieceIdentity::Queen
        } else if *behavior == rook() {
            PieceIdentity::Rook
        } else if *behavior == bishop() {
            PieceIdentity::Bishop
        } else if *behavior == knight() {
            PieceIdentity::Knight
        } else if *behavior == pawn() {
            PieceIdentity::Pawn
        } else {
            panic!("Only use classical piece behaviors here.");
        };
        let piece_icon = piece_unicode(identity, *team);
        if let Some(mut icon) = icon {
            *icon = PieceIcon::Character(piece_icon);
        } else {
            commands
                .entity(entity)
                .insert(PieceIcon::Character(piece_icon));
        }
    }
}
