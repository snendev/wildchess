use anyhow::Error as AnyError;
use thiserror::Error;

use crate::team::Team;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
use File::{A, B, C, D, E, F, G, H};

impl File {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            File::A,
            File::B,
            File::C,
            File::D,
            File::E,
            File::F,
            File::G,
            File::H,
        ]
        .into_iter()
    }
}

#[derive(Debug, Error)]
enum FileParseError {
    #[error("Invalid rank: `{0}`")]
    Int(u16),
    #[error("Invalid rank: `{0}`")]
    Char(char),
}

impl TryFrom<char> for File {
    type Error = AnyError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'a' => Ok(A),
            'b' => Ok(B),
            'c' => Ok(C),
            'd' => Ok(D),
            'e' => Ok(E),
            'f' => Ok(F),
            'g' => Ok(G),
            'h' => Ok(H),
            _ => Err(AnyError::new(FileParseError::Char(value))),
        }
    }
}

impl From<&File> for char {
    fn from(rank: &File) -> Self {
        match rank {
            A => 'a',
            B => 'b',
            C => 'c',
            D => 'd',
            E => 'e',
            F => 'f',
            G => 'g',
            H => 'h',
        }
    }
}

impl TryFrom<u16> for File {
    type Error = AnyError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(A),
            1 => Ok(B),
            2 => Ok(C),
            3 => Ok(D),
            4 => Ok(E),
            5 => Ok(F),
            6 => Ok(G),
            7 => Ok(H),
            _ => Err(AnyError::new(FileParseError::Int(value))),
        }
    }
}

impl From<&File> for u16 {
    fn from(rank: &File) -> Self {
        match rank {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
            E => 4,
            F => 5,
            G => 6,
            H => 7,
        }
    }
}

impl File {
    pub fn checked_add(&self, delta: i16) -> Option<File> {
        let current: u16 = self.into();
        current
            .checked_add_signed(delta)
            .and_then(|next| next.try_into().ok())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}
use Rank::{Eight, Five, Four, One, Seven, Six, Three, Two};

impl Rank {
    pub fn reverse(&self) -> Rank {
        match self {
            One => Eight,
            Two => Seven,
            Three => Six,
            Four => Five,
            Five => Four,
            Six => Three,
            Seven => Two,
            Eight => One,
        }
    }
}

#[derive(Debug, Error)]
enum RankParseError {
    #[error("Invalid rank: `{0}`")]
    Int(u16),
    #[error("Invalid rank: `{0}`")]
    Char(char),
}

impl TryFrom<char> for Rank {
    type Error = AnyError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1' => Ok(One),
            '2' => Ok(Two),
            '3' => Ok(Three),
            '4' => Ok(Four),
            '5' => Ok(Five),
            '6' => Ok(Six),
            '7' => Ok(Seven),
            '8' => Ok(Eight),
            _ => Err(AnyError::new(RankParseError::Char(value))),
        }
    }
}

impl From<&Rank> for char {
    fn from(rank: &Rank) -> Self {
        match rank {
            One => '1',
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8',
        }
    }
}

impl TryFrom<u16> for Rank {
    type Error = AnyError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(One),
            1 => Ok(Two),
            2 => Ok(Three),
            3 => Ok(Four),
            4 => Ok(Five),
            5 => Ok(Six),
            6 => Ok(Seven),
            7 => Ok(Eight),
            _ => Err(AnyError::new(RankParseError::Int(value))),
        }
    }
}

impl From<&Rank> for u16 {
    fn from(file: &Rank) -> Self {
        match file {
            One => 0,
            Two => 1,
            Three => 2,
            Four => 3,
            Five => 4,
            Six => 5,
            Seven => 6,
            Eight => 7,
        }
    }
}

impl Rank {
    pub fn checked_add(&self, delta: i16) -> Option<Rank> {
        let current: u16 = self.into();
        current
            .checked_add_signed(delta)
            .and_then(|next| next.try_into().ok())
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Square {
    pub rank: Rank,
    pub file: File,
}

impl Square {
    pub fn new(rank: Rank, file: File) -> Square {
        Square { rank, file }
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
        Ok(Square::new(rank.try_into()?, file.try_into()?))
    }
}

impl TryFrom<&str> for Square {
    type Error = AnyError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let mut chars = text.chars();
        (
            chars.next().ok_or(SquareParseError {
                text: text.to_string(),
            })?,
            chars.next().ok_or(SquareParseError {
                text: text.to_string(),
            })?,
        )
            .try_into()
    }
}

impl Square {
    pub fn checked_add(&self, y: i16, x: i16) -> Option<Square> {
        self.rank
            .checked_add(y)
            .zip(self.file.checked_add(x))
            .map(|(rank, file)| Square::new(rank, file))
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", char::from(&self.file), char::from(&self.rank))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocalSquare {
    pub rank: Rank,
    pub file: File,
}

impl LocalSquare {
    pub fn new(rank: Rank, file: File) -> Self {
        LocalSquare { rank, file }
    }

    pub fn to_square(&self, team: &Team) -> Square {
        Square::new(
            match team {
                Team::White => self.rank,
                Team::Black => self.rank.reverse(),
            },
            self.file,
        )
    }
}
