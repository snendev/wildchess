use anyhow::Error as AnyError;
use bevy::prelude::Component;
use thiserror::Error;

use crate::Team;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
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
    Int(u8),
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

impl TryFrom<u8> for File {
    type Error = AnyError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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

impl From<&File> for u8 {
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
    pub fn checked_add(&self, delta: i8) -> Option<File> {
        let current: u8 = self.into();
        current
            .checked_add_signed(delta)
            .map(|next| next.try_into().ok())
            .flatten()
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
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

#[derive(Debug, Error)]
enum RankParseError {
    #[error("Invalid rank: `{0}`")]
    Int(u8),
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

impl TryFrom<u8> for Rank {
    type Error = AnyError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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

impl From<&Rank> for u8 {
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
    pub fn checked_add(&self, delta: i8) -> Option<Rank> {
        let current: u8 = self.into();
        current
            .checked_add_signed(delta)
            .map(|next| next.try_into().ok())
            .flatten()
    }
}

#[derive(Clone, Component, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}

impl Square {
    pub fn new(file: File, rank: Rank) -> Square {
        Square { file, rank }
    }

    pub fn piece(file: File, team: Team) -> Square {
        Square::new(
            file,
            if team == Team::White {
                Rank::One
            } else {
                Rank::Eight
            },
        )
    }

    pub fn pawn(file: File, team: Team) -> Square {
        Square::new(
            file,
            if team == Team::White {
                Rank::Two
            } else {
                Rank::Seven
            },
        )
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
        Ok((
            chars.next().ok_or(SquareParseError {
                text: text.to_string(),
            })?,
            chars.next().ok_or(SquareParseError {
                text: text.to_string(),
            })?,
        )
            .try_into()?)
    }
}

impl Square {
    pub fn checked_add(&self, x: i8, y: i8) -> Option<Square> {
        self.file
            .checked_add(x)
            .zip(self.rank.checked_add(y))
            .map(|(file, rank)| Square::new(file, rank))
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", char::from(&self.file), char::from(&self.rank))
    }
}
