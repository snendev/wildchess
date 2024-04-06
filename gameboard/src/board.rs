use crate::BoardVector;

pub trait GameBoard: Default + Sized {
    type Vector: BoardVector;

    fn is_in_bounds(&self, position: Self::Vector) -> bool;

    fn scan(&self, origin: Self::Vector, step: Self::Vector) -> BoardIterator<'_, Self> {
        BoardIterator {
            board: self,
            current_position: origin,
            step,
        }
    }
}

pub struct BoardIterator<'a, B: GameBoard> {
    board: &'a B,
    current_position: B::Vector,
    step: B::Vector,
}

impl<'a, B: GameBoard> Iterator for BoardIterator<'a, B> {
    // TODO maybe iter returns (Square, DoesCollide)
    type Item = B::Vector;

    fn next(&mut self) -> Option<Self::Item> {
        let next_position = self.current_position + self.step;
        if self.board.is_in_bounds(next_position) {
            self.current_position = next_position;
            Some(self.current_position)
        } else {
            None
        }
    }
}
