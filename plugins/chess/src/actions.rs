use serde::{Deserialize, Serialize};

use bevy::ecs::entity::MapEntities;
use bevy::prelude::{Component, Entity, EntityMapper, Reflect};
use bevy::utils::{HashMap, HashSet};

use crate::{board::Square, pattern::Pattern, pieces::Orientation};

#[derive(Clone, Debug, Default, PartialEq)]
#[derive(Reflect)]
#[derive(Deserialize, Serialize)]
pub struct Movement {
    pub from: Square,
    pub to: Square,
    pub orientation: Orientation,
}

impl Movement {
    pub fn new(from: Square, to: Square, orientation: Orientation) -> Self {
        Self {
            from,
            to,
            orientation,
        }
    }

    pub fn from(&self) -> Square {
        self.from
    }

    pub fn to(&self) -> Square {
        self.to
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[derive(Reflect)]
#[derive(Deserialize, Serialize)]
pub struct Action {
    pub movement: Movement,
    pub side_effects: Vec<(Entity, Movement)>,
    pub scanned_squares: Vec<Square>,
    pub using_pattern: Option<Pattern>,
    pub captures: HashSet<Square>,
    pub threats: HashSet<Square>,
}

impl Action {
    pub fn movement(
        from_square: Square,
        landing_square: Square,
        landing_orientation: Orientation,
        scanned_squares: Vec<Square>,
        pattern: Option<Pattern>,
    ) -> Self {
        Action {
            movement: Movement {
                from: from_square,
                to: landing_square,
                orientation: landing_orientation,
            },
            scanned_squares,
            using_pattern: pattern,
            ..Default::default()
        }
    }
}

impl MapEntities for Action {
    fn map_entities<M: EntityMapper>(&mut self, mapper: &mut M) {
        self.side_effects = self
            .side_effects
            .iter()
            .map(|(entity, movement)| (mapper.map_entity(*entity), movement.clone()))
            .collect();
    }
}

// TODO: Refactor actions
// There are a number of options to improve here:
// - entity observers to control attaching actions
// - Vec<Action> to account for multiple options
// - additionally include promotions as unique options
#[derive(Clone, Debug, Default)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct Actions(pub HashMap<Square, Action>);

impl Actions {
    pub fn new(map: HashMap<Square, Action>) -> Self {
        Actions(map)
    }

    pub fn get(&self, square: &Square) -> Option<&Action> {
        self.0.get(square)
    }

    // TODO: currently no good way to handle colliding squares
    pub fn extend(&mut self, additional_targets: Self) {
        self.0.extend(additional_targets.0);
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }
}

impl MapEntities for Actions {
    fn map_entities<M: EntityMapper>(&mut self, mapper: &mut M) {
        for (_, action) in self.0.iter_mut() {
            action.map_entities(mapper);
        }
    }
}

#[derive(Clone, Debug)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct LastAction(pub Action);

impl MapEntities for LastAction {
    fn map_entities<M: EntityMapper>(&mut self, mapper: &mut M) {
        self.0.map_entities(mapper);
    }
}
