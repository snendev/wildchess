use anyhow::Error as AnyError;
use thiserror::Error;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::{pieces::Orientation, team::Team};

use super::Board;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct File(pub u16);

impl File {
    pub const A: Self = File(0);
    pub const B: Self = File(1);
    pub const C: Self = File(2);
    pub const D: Self = File(3);
    pub const E: Self = File(4);
    pub const F: Self = File(5);
    pub const G: Self = File(6);
    pub const H: Self = File(7);
    pub const I: Self = File(8);

    pub fn checked_add(&self, delta: i16) -> Option<File> {
        let current: u16 = self.into();
        current
            .checked_add_signed(delta)
            .and_then(|next| next.try_into().ok())
    }

    // TODO: safer arithmetic
    pub fn reverse(&self, max: File) -> File {
        File(max.0 - self.0)
    }
}

#[derive(Debug, Error)]
enum FileParseError {
    #[error("Invalid rank: `{0}`")]
    Char(char),
}

impl TryFrom<char> for File {
    type Error = AnyError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'a' => Ok(File::A),
            'b' => Ok(File::B),
            'c' => Ok(File::C),
            'd' => Ok(File::D),
            'e' => Ok(File::E),
            'f' => Ok(File::F),
            'g' => Ok(File::G),
            'h' => Ok(File::H),
            'i' => Ok(File::I),
            _ => Err(AnyError::new(FileParseError::Char(value))),
        }
    }
}

impl From<&File> for char {
    fn from(file: &File) -> Self {
        match file {
            File(0) => 'a',
            File(1) => 'b',
            File(2) => 'c',
            File(3) => 'd',
            File(4) => 'e',
            File(5) => 'f',
            File(6) => 'g',
            File(7) => 'h',
            File(8) => 'i',
            _ => 'x',
        }
    }
}

impl From<u16> for File {
    fn from(value: u16) -> Self {
        File(value)
    }
}

impl From<&File> for u16 {
    fn from(file: &File) -> Self {
        file.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Rank(pub u16);

impl Rank {
    // common chess aliases
    // TODO: should probably extrapolate
    pub const ONE: Self = Rank(0);
    pub const TWO: Self = Rank(1);
    pub const THREE: Self = Rank(2);
    pub const FOUR: Self = Rank(3);
    pub const FIVE: Self = Rank(4);
    pub const SIX: Self = Rank(5);
    pub const SEVEN: Self = Rank(6);
    pub const EIGHT: Self = Rank(7);
    pub const NINE: Self = Rank(8);

    pub fn checked_add(&self, delta: i16) -> Option<Rank> {
        let current: u16 = self.into();
        current
            .checked_add_signed(delta)
            .and_then(|next| next.try_into().ok())
    }

    // TODO: safer arithmetic
    pub fn reverse(&self, max: Rank) -> Rank {
        Rank(max.0 - self.0)
    }

    // TODO: is this the right place for this code?
    pub fn back_rank(team: Team, board: &Board) -> Rank {
        Square::new(File::H, Rank::EIGHT)
            .reorient(team.orientation(), board)
            .rank
    }
}

#[derive(Debug, Error)]
enum RankParseError {
    #[error("Invalid rank: `{0}`")]
    Char(char),
}

impl TryFrom<char> for Rank {
    type Error = AnyError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1' => Ok(Rank(0)),
            '2' => Ok(Rank(1)),
            '3' => Ok(Rank(2)),
            '4' => Ok(Rank(3)),
            '5' => Ok(Rank(4)),
            '6' => Ok(Rank(5)),
            '7' => Ok(Rank(6)),
            '8' => Ok(Rank(7)),
            '9' => Ok(Rank(8)),
            _ => Err(AnyError::new(RankParseError::Char(value))),
        }
    }
}

impl From<&Rank> for char {
    fn from(rank: &Rank) -> Self {
        match rank.0 {
            0..=8 => char::from_digit((rank.0 + 1).into(), 10).unwrap(),
            100 => '?',
            _ => unimplemented!("need more work to support arbitrary rank strings"),
        }
    }
}

impl From<u16> for Rank {
    fn from(value: u16) -> Self {
        Rank(value)
    }
}

impl From<&Rank> for u16 {
    fn from(rank: &Rank) -> Self {
        rank.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}

impl Square {
    pub fn new(file: File, rank: Rank) -> Square {
        Square { file, rank }
    }
}

#[derive(Debug, Error)]
#[error("Invalid square: `{text}`")]
struct SquareParseError {
    text: String,
}

impl TryFrom<(char, char)> for Square {
    type Error = AnyError;

    fn try_from((file, rank): (char, char)) -> Result<Self, Self::Error> {
        Ok(Square::new(file.try_into()?, rank.try_into()?))
    }
}

impl TryFrom<&str> for Square {
    type Error = AnyError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let mut chars = text.chars();
        let file = chars.next().ok_or(SquareParseError {
            text: text.to_string(),
        })?;
        let rank = chars.next().ok_or(SquareParseError {
            text: text.to_string(),
        })?;
        (file, rank).try_into()
    }
}

impl Square {
    // whether this is an "even" square in the grid
    // where (0,0) (even) is the bottom-left corner
    pub fn is_even(&self) -> bool {
        (self.file.0 + self.rank.0) % 2 == 0
    }

    fn is_in_bounds(&self, max: &Square) -> bool {
        self.rank <= max.rank && self.file <= max.file
    }

    // TODO: share more code with Orientation::orient
    pub fn reorient(&self, orientation: Orientation, board: &Board) -> Self {
        match orientation {
            Orientation::Up => Square::new(self.file, self.rank),
            Orientation::Down => Square::new(self.file, self.rank.reverse(board.size.rank)),
            Orientation::Left => Square::new(File(self.rank.0), Rank(self.file.0)),
            Orientation::Right => {
                Square::new(File(self.file.reverse(board.size.file).0), self.rank)
            }
        }
    }

    pub fn checked_add(&self, x: i16, y: i16, max: &Square) -> Option<Square> {
        self.rank
            .checked_add(y)
            .zip(self.file.checked_add(x))
            .map(|(rank, file)| Square::new(file, rank))
            .and_then(|square| {
                if square.is_in_bounds(max) {
                    Some(square)
                } else {
                    None
                }
            })
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", char::from(&self.file), char::from(&self.rank))
    }
}

// convenient alias
impl From<(File, Rank)> for Square {
    fn from((file, rank): (File, Rank)) -> Self {
        Square { file, rank }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;

    use super::*;

    #[test]
    fn test_addition() {
        let board_size = Board::chess_board().size;
        assert_eq!(
            Square::new(File::A, Rank::ONE).checked_add(3, 7, &board_size),
            Some(Square::new(File::D, Rank::EIGHT)),
        );

        assert_eq!(
            Square::new(File::D, Rank::FIVE).checked_add(1, 0, &board_size),
            Some(Square::new(File::E, Rank::FIVE)),
        );

        assert_eq!(
            Square::new(File::B, Rank::TWO).checked_add(-1, 0, &board_size),
            Some(Square::new(File::A, Rank::TWO)),
        );

        assert_eq!(
            Square::new(File::G, Rank::FOUR).checked_add(0, 3, &board_size),
            Some(Square::new(File::G, Rank::SEVEN)),
        );

        assert_eq!(
            Square::new(File::G, Rank::FOUR).checked_add(0, -3, &board_size),
            Some(Square::new(File::G, Rank::ONE)),
        );

        // out of bounds checks
        assert_eq!(
            Square::new(File::A, Rank::ONE).checked_add(-1, 0, &board_size),
            None,
        );
        assert_eq!(
            Square::new(File::A, Rank::ONE).checked_add(0, -1, &board_size),
            None,
        );
        assert_eq!(
            Square::new(File::H, Rank::EIGHT).checked_add(1, 0, &board_size),
            None,
        );
        assert_eq!(
            Square::new(File::H, Rank::EIGHT).checked_add(0, 1, &board_size),
            None,
        );
    }
}
