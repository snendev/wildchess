use serde::{Deserialize, Serialize};

use bevy_ecs::prelude::Component;
#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

// A name for the "kind" of piece this is. Usually relates to a specific set of behaviors,
// but variants often change these.
// It is mostly useful for supplying contextual information to users, such as displaying a
// particular icon.
#[derive(Clone, Copy, Debug, Default)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub enum PieceIdentity {
    // The default: implies that this piece does not fit an existing stereotype
    // #[default]
    // Wild,
    // Somewhat universal
    King,
    // Chess
    Queen,
    Rook,
    Bishop,
    Knight,
    #[default]
    Pawn,
    // TODO: Shogi
    // TODO: Xiangqi
    // TODO: others
}
