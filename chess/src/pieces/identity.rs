use bevy::prelude::{Component, Reflect, ReflectComponent};

// A name for the "kind" of piece this is. Usually relates to a specific set of behaviors,
// but variants often change these.
// It is mostly useful for supplying contextual information to users, such as displaying a
// particular icon.
#[derive(Clone, Copy, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub enum PieceIdentity {
    #[default]
    Wild,
    // Somewhat universal
    King,
    // Chess
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    // TODO: Shogi
    // TODO: Xiangqi
    // TODO: others
}
