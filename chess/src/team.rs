use bevy::prelude::{Component, Reflect, ReflectComponent};

#[derive(Clone, Copy, Component, Debug, Default, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub enum Team {
    #[default]
    White,
    Black,
}
