use bevy_ecs::prelude::Component;
#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::Reflect;

#[derive(Clone, Copy, Component, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub enum Team {
    #[default]
    White,
    Black,
}
