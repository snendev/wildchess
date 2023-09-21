mod king;
mod pawn;
mod piece;

pub use piece::{AdvancedBuilder, EliteBuilder, LegendaryBuilder, MajorBuilder, MinorBuilder};

// Piece identity described by the starting squares
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PieceBuilder;
