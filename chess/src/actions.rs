use bevy::{
    prelude::{Component, Reflect, ReflectComponent},
    utils::HashMap,
};

use crate::{board::Square, pattern::Pattern, pieces::Orientation};

#[derive(Clone, Debug, Default, PartialEq, Reflect)]
pub struct Action {
    pub from_square: Square,
    pub landing_square: Square,
    pub landing_orientation: Orientation,
    pub scanned_squares: Vec<Square>,
    pub using_pattern: Pattern,
    pub captures: Vec<Square>,
}

impl Action {
    pub fn movement(
        from_square: Square,
        landing_square: Square,
        landing_orientation: Orientation,
        scanned_squares: Vec<Square>,
        pattern: Pattern,
    ) -> Self {
        Action {
            from_square,
            landing_square,
            landing_orientation,
            captures: vec![],
            scanned_squares,
            using_pattern: pattern,
        }
    }

    pub fn capture(
        from_square: Square,
        landing_square: Square,
        landing_orientation: Orientation,
        scanned_squares: Vec<Square>,
        pattern: Pattern,
        captures: Vec<Square>,
    ) -> Self {
        Action {
            from_square,
            landing_square,
            landing_orientation,
            scanned_squares,
            using_pattern: pattern,
            captures,
        }
    }
}

#[derive(Clone, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Actions(HashMap<Square, Action>);

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
