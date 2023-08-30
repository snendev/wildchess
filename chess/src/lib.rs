use bevy::prelude::Commands;

pub mod pieces;
pub mod square;
pub mod team;

pub trait ChessBoard {
    fn setup_board(commands: Commands);
}
