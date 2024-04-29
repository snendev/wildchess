use anyhow::Error as AnyError;
use derive_more::{Add, Mul};
use thiserror::Error;

use bevy_derive::Deref;
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Add, Mul)]
#[derive(Deref)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct File(i16);

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
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Add, Mul)]
#[derive(Deref)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Rank(i16);

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
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[derive(Add, Mul)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Square {
    file: File,
    rank: Rank,
}

impl Square {
    pub const ZERO: Square = Square {
        file: File::A,
        rank: Rank::ONE,
    };

    pub fn new(file: File, rank: Rank) -> Self {
        Self { file, rank }
    }

    pub fn from_values(file: i16, rank: i16) -> Self {
        Self {
            file: File(file),
            rank: Rank(rank),
        }
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn file(&self) -> File {
        self.file
    }
}

impl From<i16> for File {
    fn from(value: i16) -> Self {
        File(value)
    }
}

impl From<i16> for Rank {
    fn from(value: i16) -> Self {
        Rank(value)
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
            0..=8 => char::from_digit((rank.0 as u32 + 1).into(), 10).unwrap(),
            100 => '?',
            _ => unimplemented!("need more work to support arbitrary rank strings"),
        }
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
