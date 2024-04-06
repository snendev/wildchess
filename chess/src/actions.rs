use bevy::{
    prelude::{Component, Reflect, ReflectComponent},
    utils::{HashMap, HashSet},
};

use fairy_gameboard::{GameBoard, BoardVector,Movement};

#[derive(Clone, Debug, Default)]
pub struct Action<B: GameBoard> {
    // the set of all moved pieces
    pub movements: Vec<Movement<B>>,
    // the set of all squares "touched" in the path
    pub path: Vec<B::Vector>,
    // the set of all positions that are captured by this action
    pub captures: HashSet<B::Vector>,
}

impl<B: GameBoard> Action<B> {
    pub fn movement(
        from: B::Vector,
        to: B::Vector,
        landing_orientation: <B::Vector as BoardVector>::Symmetry,
        path: Vec<B::Vector>,
    ) -> Self {
        Action {
            movements: vec![Movement::new(from, to, landing_orientation)],
            path,
            ..Default::default()
        }
    }
}

// #[derive(Clone, Debug, Default, PartialEq, Reflect)]
// pub struct Action {
//     pub movement: Movement,
//     pub side_effects: Vec<(Entity, Movement)>,
//     pub scanned_squares: Vec<Square>,
//     pub using_pattern: Option<Pattern>,
//     pub captures: HashSet<Square>,
//     pub threats: HashSet<Square>,
// }

// impl Action {
//     pub fn movement(
//         from_square: Square,
//         landing_square: Square,
//         landing_orientation: Orientation,
//         scanned_squares: Vec<Square>,
//         pattern: Option<Pattern>,
//     ) -> Self {
//         Action {
//             movement: Movement {
//                 from: from_square,
//                 to: landing_square,
//                 orientation: landing_orientation,
//             },
//             scanned_squares,
//             using_pattern: pattern,
//             ..Default::default()
//         }
//     }
// }

#[derive(Clone, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Actions<B: GameBoard>(pub HashMap<B::Vector, Action<B>>);

impl<B: GameBoard> Actions <B>{
    pub fn new(map: HashMap<B::Vector, Action<B>>) -> Self {
        Actions(map)
    }

    pub fn get(&self, square: &B::Vector) -> Option<&Action<B>> {
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
