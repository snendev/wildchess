use bevy::prelude::{Component, Reflect, ReflectComponent};

// Once all Royal pieces are captured, a player loses the game.
#[derive(Clone, Copy, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Royal;
