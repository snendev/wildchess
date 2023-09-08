use chess::{
    pieces::{Behavior, Mutation, MutationCondition, Position, Royal},
    square::{File, Rank},
    team::Team,
};

use super::{pieces, ClassicalIdentity, ClassicalPiece};
use crate::utils;

pub(crate) struct ClassicalLayout;

impl ClassicalLayout {
    pub(crate) fn pieces() -> Vec<(ClassicalPiece, Position)> {
        utils::pieces_by_team(utils::team_piece_square(Rank::One, File::A), |team| {
            rook(team)
        })
        .chain(utils::pieces_by_team(
            utils::team_piece_square(Rank::One, File::H),
            |team| rook(team),
        ))
        .chain(utils::pieces_by_team(
            utils::team_piece_square(Rank::One, File::B),
            |team| knight(team),
        ))
        .chain(utils::pieces_by_team(
            utils::team_piece_square(Rank::One, File::G),
            |team| knight(team),
        ))
        .chain(utils::pieces_by_team(
            utils::team_piece_square(Rank::One, File::C),
            |team| bishop(team),
        ))
        .chain(utils::pieces_by_team(
            utils::team_piece_square(Rank::One, File::F),
            |team| bishop(team),
        ))
        .chain(utils::pieces_by_team(
            utils::team_piece_square(Rank::One, File::D),
            |team| queen(team),
        ))
        .chain(utils::pieces_by_team(
            utils::team_piece_square(Rank::One, File::E),
            |team| king(team),
        ))
        .chain(File::all().flat_map(|file| {
            utils::pieces_by_team(utils::team_piece_square(Rank::Two, file), |team| pawn(team))
        }))
        .collect()
    }
}

fn king(team: Team) -> ClassicalPiece {
    ClassicalPiece {
        behavior: pieces::king(),
        team,
        royal: Some(Royal),
        extra: ClassicalIdentity::King,
        ..Default::default()
    }
}

fn pawn(team: Team) -> ClassicalPiece {
    ClassicalPiece {
        behavior: pieces::pawn(),
        team,
        mutation: Some(Mutation::<ClassicalIdentity> {
            condition: MutationCondition::Rank(utils::team_local_rank(team, Rank::One)),
            options: vec![queen(team), rook(team), bishop(team), knight(team)],
        }),
        extra: ClassicalIdentity::Pawn,
        ..Default::default()
    }
}

fn piece(behavior: Behavior, team: Team, identity: ClassicalIdentity) -> ClassicalPiece {
    ClassicalPiece {
        behavior,
        team,
        extra: identity,
        ..Default::default()
    }
}

fn rook(team: Team) -> ClassicalPiece {
    piece(pieces::rook(), team, ClassicalIdentity::Rook)
}

fn knight(team: Team) -> ClassicalPiece {
    piece(pieces::knight(), team, ClassicalIdentity::Knight)
}

fn bishop(team: Team) -> ClassicalPiece {
    piece(pieces::bishop(), team, ClassicalIdentity::Bishop)
}

fn queen(team: Team) -> ClassicalPiece {
    piece(pieces::queen(), team, ClassicalIdentity::Queen)
}
