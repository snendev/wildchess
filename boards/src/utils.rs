use chess::{
    pieces::{PieceDefinition, Position},
    square::{File, Rank, Square},
    team::Team::{self, Black, White},
};

const TEAMS: [Team; 2] = [White, Black];

pub(crate) fn pieces_by_team<'a, Extra: Default>(
    pick_square: impl Fn(Team) -> Square + 'a,
    pick_piece: impl Fn(Team) -> PieceDefinition<Extra> + 'a,
) -> impl Iterator<Item = (PieceDefinition<Extra>, Position)> + 'a {
    TEAMS.into_iter().map(move |team| {
        let square = pick_square(team);
        let piece = pick_piece(team);
        (piece, Position(square))
    })
}

pub(crate) fn team_local_rank(team: Team, local_rank: Rank) -> Rank {
    match (team, local_rank) {
        (Team::White, local_rank) => local_rank,
        (Team::Black, local_rank) => local_rank.reverse(),
    }
}

pub(crate) fn team_piece_square(local_rank: Rank, file: File) -> impl Fn(Team) -> Square {
    move |team: Team| (file, team_local_rank(team, local_rank)).into()
}
