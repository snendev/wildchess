use bevy_ecs::prelude::Component;
#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

// Once all Royal pieces are captured, a player loses the game.
#[derive(Clone, Copy, Debug, Default)]
#[derive(Component)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub struct Royal;
