use bevy::prelude::Component;

#[derive(Clone, Copy, Component, Debug, PartialEq, Eq, Hash)]
pub enum Team {
    White,
    Black,
}
