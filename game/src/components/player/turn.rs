use bevy::prelude::Component;

#[derive(Clone, Copy, Component, Debug)]
pub enum Turn {
    Move,
    Promote,
}
