use chess::{
    board::{File, Rank, Square},
    team::Team::{self, Black, White},
};

pub const TEAMS: [Team; 2] = [White, Black];

pub fn squares_by_team(
    distance_from_back_rank: u16,
    files: impl Iterator<Item = File> + Clone,
) -> impl Iterator<Item = (Team, Square)> {
    TEAMS.into_iter().flat_map(move |team: Team| {
        let back_rank = match team {
            Team::White => Rank::ONE.0 + distance_from_back_rank,
            Team::Black => Rank::EIGHT.0 - distance_from_back_rank,
        };
        files
            .clone()
            .map(move |file| (team, Square::new(file, Rank(back_rank))))
    })
}
