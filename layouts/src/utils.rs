use chess::{
    board::{Board, File, Rank, Square},
    team::Team::{self, Black, White},
};

pub const TEAMS: [Team; 2] = [White, Black];

pub fn squares_by_team<'a>(
    distance_from_back_rank: u16,
    board: &'a Board,
    files: impl Iterator<Item = File> + Clone + 'a,
) -> impl Iterator<Item = (Team, Square)> + 'a {
    TEAMS.into_iter().flat_map(move |team: Team| {
        let rank = Square::new(File(distance_from_back_rank), Rank(distance_from_back_rank))
            .reorient(team.orientation(), board)
            .rank;
        files
            .clone()
            .map(move |file| (team, Square::new(file, rank)))
    })
}
