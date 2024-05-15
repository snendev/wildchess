use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use bevy_ecs::{
    entity::MapEntities,
    prelude::{Changed, Commands, Component, Entity, EntityMapper, Query, RemovedComponents, With},
};
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use chess::actions::Action;

use super::{Game, InGame};

#[derive(Clone, Copy, Debug)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
pub struct HasTurn;

#[derive(Clone, Debug)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
pub struct LastMove(pub Action);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Ply(usize);

impl Ply {
    pub fn new(ply: usize) -> Self {
        Ply(ply)
    }

    pub fn decrement(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }

    pub fn increment(&mut self) {
        self.0 = self.0.saturating_add(1);
    }
}

// A vector using Ply as an index.
// It tracks the action made each ply.
#[derive(Clone, Debug, Default)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct ActionHistory(Vec<(Entity, Action)>);

impl std::ops::Index<Ply> for ActionHistory {
    type Output = (Entity, Action);

    fn index(&self, index: Ply) -> &Self::Output {
        &self.0[index.0]
    }
}

impl ActionHistory {
    pub fn push(&mut self, entity: Entity, action: Action) {
        self.0.push((entity, action));
    }

    pub fn iter(&self) -> impl Iterator<Item = &(Entity, Action)> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl MapEntities for ActionHistory {
    fn map_entities<M: EntityMapper>(&mut self, mapper: &mut M) {
        self.0 = self
            .0
            .iter()
            .map(|(entity, action)| (mapper.map_entity(*entity), action.clone()))
            .collect()
    }
}

// A sparse vector using Ply as an index.
// It is kept sparse in order to minimize cloning.
#[derive(Clone, Debug)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct History<T>(BTreeMap<Ply, Option<T>>);

impl<T> History<T> {
    pub fn new() -> Self {
        History(BTreeMap::new())
    }

    pub fn get(&self, index: &Ply) -> Option<&T> {
        self.0.get(index).and_then(|value| value.as_ref())
    }

    pub fn insert(&mut self, index: Ply, value: Option<T>) {
        self.0.insert(index, value);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Ply, Option<&T>)> {
        self.0.iter().map(|(ply, value)| (ply, value.as_ref()))
    }

    // Used to get the most recent value of a ply less than or equal to the one provided
    // for example, if a piece is mutated twice, once on Ply 3 and once on Ply 8.
    // If a board viewer switches from Ply 9 to Ply 7, the viewer will need to "scroll back"
    // to Ply 3 to find the last behavior.
    // This also applies to "fast-forwarding" state.
    pub fn get_previous_nearest(&self, ply: &Ply) -> Option<&T> {
        let mut latest_ply = *ply;
        while self.0.get(&latest_ply).is_none() && latest_ply.0 > 0 {
            latest_ply.0 -= 1;
        }
        self.0.get(&latest_ply).and_then(|value| value.as_ref())
    }

    // When retrieving the most up-to-date value, rely on BTreeMap methods instead.
    pub fn get_latest(&self) -> Option<&T> {
        self.0
            .last_key_value()
            .and_then(|(_, value)| value.as_ref())
    }

    pub fn track_component_system(
        mut commands: Commands,
        game_query: Query<&Ply, With<Game>>,
        mut history_query: Query<(&InGame, Option<&mut History<T>>)>,
        updated_data_query: Query<(Entity, &T), Changed<T>>,
        mut removed_data_entities: RemovedComponents<T>,
    ) where
        T: Component + Clone,
    {
        for (entity, value) in updated_data_query.iter() {
            let Ok((in_game, history)) = history_query.get_mut(entity) else {
                continue;
            };
            let Ok(&ply) = game_query.get(in_game.0) else {
                continue;
            };
            if let Some(mut history) = history {
                history.insert(ply, Some(value.clone()));
            } else {
                let mut history = History::default();
                history.insert(ply, Some(value.clone()));
                commands.entity(entity).insert(history);
            }
        }
        for entity in removed_data_entities.read() {
            let Ok((in_game, history)) = history_query.get_mut(entity) else {
                continue;
            };
            let Ok(&ply) = game_query.get(in_game.0) else {
                continue;
            };
            if let Some(mut history) = history {
                history.insert(ply, None);
            }
        }
    }
}

impl<T> Default for History<T> {
    fn default() -> Self {
        History::new()
    }
}

// In case we track History for anything that carries Entity data
impl<T: Clone + MapEntities> MapEntities for History<T> {
    fn map_entities<M: EntityMapper>(&mut self, mapper: &mut M) {
        self.0 = self
            .0
            .iter()
            .map(|(ply, option)| {
                (
                    *ply,
                    option.as_ref().map(|value| {
                        let mut new_value = value.clone();
                        new_value.map_entities(mapper);
                        new_value
                    }),
                )
            })
            .collect()
    }
}
