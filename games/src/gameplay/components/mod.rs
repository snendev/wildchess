use serde::{Deserialize, Serialize};

#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
use bevy_ecs::{
    entity::MapEntities,
    prelude::{Component, Entity, EntityMapper},
};
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::Reflect;

mod game;
pub use game::{
    AntiGame, Atomic, ClockConfiguration, Crazyhouse, Game, GameBoard, GameSpawner, WinCondition,
};
mod clock;
pub use clock::Clock;

mod turns;
pub use turns::{ActionHistory, HasTurn, History, LastMove, Ply};

#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub struct InGame(pub Entity);

impl MapEntities for InGame {
    fn map_entities<M: EntityMapper>(&mut self, mapper: &mut M) {
        self.0 = mapper.map_entity(self.0);
    }
}
