use bevy::{
    prelude::{Component, Entity, Reflect, ReflectComponent},
    utils::{HashMap, HashSet},
};

use crate::{board::Square, pattern::Pattern, pieces::Orientation};

#[derive(Clone, Debug, Default, PartialEq, Reflect)]
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

#[derive(Clone, Debug, Default, PartialEq, Reflect)]
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

#[derive(Clone, Component, Debug, Default, Reflect)]
#[reflect(Component)]
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
