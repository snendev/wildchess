use games::chess::{
    pieces::PieceIdentity::{self, Bishop, King, Knight, Pawn, Queen, Rook},
    team::Team::{self, Black, White},
};

pub(crate) fn piece_unicode(piece: &PieceIdentity, team: &Team) -> char {
    match (piece, team) {
        (King, White) => '\u{2654}',
        (King, Black) => '\u{265A}',
        (Queen, White) => '\u{2655}',
        (Queen, Black) => '\u{265B}',
        (Rook, White) => '\u{2656}',
        (Rook, Black) => '\u{265C}',
        (Bishop, White) => '\u{2657}',
        (Bishop, Black) => '\u{265D}',
        (Knight, White) => '\u{2658}',
        (Knight, Black) => '\u{265E}',
        (Pawn, White) => '\u{2659}',
        (Pawn, Black) => '\u{265F}',
        _ => 'X',
    }
}
