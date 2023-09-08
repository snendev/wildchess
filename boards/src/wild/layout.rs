use rand::{thread_rng, Rng};

use chess::{
    pieces::{Behavior, Mutation, MutationCondition, PieceDefinition, Position, Royal},
    square::{File, Rank},
    team::Team,
};

use crate::{utils, wild::PieceKind};

pub struct WildLayout;

impl WildLayout {
    pub fn pieces() -> Vec<(PieceDefinition, Position)> {
        let piece_set = random_pieces();

        let pawn_promotion_options = vec![
            piece_set.pieces.0.clone(),
            piece_set.pieces.1.clone(),
            piece_set.pieces.2.clone(),
            piece_set.pieces.3.clone(),
        ];

        // typical pieces
        utils::pieces_by_team(utils::team_piece_square(Rank::One, File::A), |team| {
            piece(piece_set.pieces.0.clone(), team)
        })
        .chain(utils::pieces_by_team(
            utils::team_piece_square(Rank::One, File::H),
            |team| piece(piece_set.pieces.0.clone(), team),
        ))
        .chain(utils::pieces_by_team(
            utils::team_piece_square(Rank::One, File::B),
            |team| piece(piece_set.pieces.1.clone(), team),
        ))
        .chain(utils::pieces_by_team(
            utils::team_piece_square(Rank::One, File::G),
            |team| piece(piece_set.pieces.1.clone(), team),
        ))
        .chain(utils::pieces_by_team(
            utils::team_piece_square(Rank::One, File::C),
            |team| piece(piece_set.pieces.2.clone(), team),
        ))
        .chain(utils::pieces_by_team(
            utils::team_piece_square(Rank::One, File::F),
            |team| piece(piece_set.pieces.2.clone(), team),
        ))
        .chain(utils::pieces_by_team(
            utils::team_piece_square(Rank::One, File::D),
            |team| piece(piece_set.pieces.3.clone(), team),
        ))
        .chain(utils::pieces_by_team(
            utils::team_piece_square(Rank::One, File::E),
            |team| king(piece_set.king.clone(), team),
        ))
        .chain(File::all().flat_map(|file| {
            utils::pieces_by_team(utils::team_piece_square(Rank::Two, file), |team| {
                pawn(
                    piece_set.pawn.clone(),
                    team,
                    pawn_promotion(team, pawn_promotion_options.clone()),
                )
            })
        }))
        .collect()
    }
}

fn piece(behavior: Behavior, team: Team) -> PieceDefinition {
    PieceDefinition {
        behavior,
        team,
        ..Default::default()
    }
}

fn king(behavior: Behavior, team: Team) -> PieceDefinition {
    PieceDefinition {
        behavior,
        team,
        royal: Some(Royal),
        ..Default::default()
    }
}

fn pawn(behavior: Behavior, team: Team, mutation: Mutation) -> PieceDefinition {
    PieceDefinition {
        behavior,
        team,
        mutation: Some(mutation),
        ..Default::default()
    }
}

fn pawn_promotion(team: Team, options: Vec<Behavior>) -> Mutation {
    Mutation {
        condition: MutationCondition::Rank(match team {
            Team::White => Rank::Eight,
            Team::Black => Rank::One,
        }),
        options: options
            .into_iter()
            .map(|behavior| PieceDefinition {
                behavior,
                team,
                ..Default::default()
            })
            .collect(),
    }
}

struct WildPieceSet {
    pub pieces: (Behavior, Behavior, Behavior, Behavior),
    pub pawn: Behavior,
    pub king: Behavior,
}

fn random_pieces() -> WildPieceSet {
    let mut rng = thread_rng();
    let max_value: u32 = rng.gen_range(50..80);
    let mut current_value: u32 = 0;

    // pieces
    let ah = PieceKind::generate_piece(max_value, &mut current_value);
    let bg = PieceKind::generate_piece(max_value, &mut current_value);
    let cf = PieceKind::generate_piece(max_value, &mut current_value);
    let d = PieceKind::generate_piece(max_value, &mut current_value);
    // pawns
    let pawn = PieceKind::generate_pawn();
    // king
    let king = PieceKind::generate_king();

    WildPieceSet {
        pieces: (ah, bg, cf, d),
        pawn,
        king,
    }
}
