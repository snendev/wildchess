use bevy::{
    prelude::{Component, Entity, Reflect, ReflectComponent},
    utils::{HashMap, HashSet},
};

use crate::GameBoard;

#[derive(Clone, Debug, Default, PartialEq)]
#[derive(Reflect)]
pub struct Movement<B: GameBoard> {
    pub from: B::Position,
    pub to: B::Position,
    pub orientation: B::Axes,
}

impl <B: GameBoard> Movement<B> {
    pub fn new(from: B::Position, to: B::Position, orientation: B::Axes) -> Self {
        Self {
            from,
            to,
            orientation,
        }
    }

    pub fn from(&self) -> B::Position {
        self.from
    }

    pub fn to(&self) -> B::Position {
        self.to
    }

    pub fn orientation(&self) -> B::Axes {
        self.orientation
    }
}

#[derive(Clone, Debug, Default)]
pub struct Action<B:GameBoard> {
    pub movement: Movement<B>,
    pub side_effects: Vec<(Entity, Movement<B>)>,
    pub path: Vec<B::Position>,
    pub captures: HashSet<B::Position>,
    pub threats: HashSet<B::Position>,
}

impl <B: GameBoard>Action<B> {
    pub fn movement(
        from: B::Position,
        to: B::Position,
        landing_orientation: B::Axes,
        path: Vec<B::Position>,
    ) -> Self {
        Action {
            movement: Movement {
                from,
                to,
                orientation: landing_orientation,
            },
            path,
            ..Default::default()
        }
    }
}

#[derive(Clone, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Actions<B:GameBoard>(pub HashMap<B::Position, Action<B>>);

impl <B:GameBoard> Actions<B> {
    pub fn new(map: HashMap<B::Position, Action<B>>) -> Self {
        Actions(map)
    }

    pub fn get(&self, position: &B::Position) -> Option<&Action<B>> {
        self.0.get(position)
    }

    // TODO: currently no good way to handle colliding squares
    pub fn extend(&mut self, additional_targets: Self) {
        self.0.extend(additional_targets.0);
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }
}
