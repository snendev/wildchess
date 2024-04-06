#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::GameBoard;

// The calculation type for board searches
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Scanner<B: GameBoard> {
    // how many steps can this pattern be executed for?
    // if None, do not set a limit
    pub range: Option<Range>,
    // configuration for how to deal with colliders
    pub collisions: PatternCollisions,
    // which squares this pattern can be activated from, if any
    pub only_from_positions: Vec<B::Vector>,
    // which squares this pattern must filter out, if any
    pub forbidden_targets: Vec<B::Vector>,
}

impl<B: GameBoard> Scanner<B> {
    pub fn with_range(mut self, range: Range) -> Self {
        self.range = Some(range);
        self
    }

    // how to handle collisions

    pub fn with_collision_behavior(mut self, collisions: PatternCollisions) -> Self {
        self.collisions = collisions;
        self
    }

    pub fn only_from_positions(mut self, allowed_positions: Vec<B::Vector>) {
        self.only_from_positions = allowed_positions;
    }

    pub fn with_forbidden_targets(mut self, forbidden_targets: Vec<B::Vector>) {
        self.forbidden_targets = forbidden_targets;
    }


    pub fn scan_board<'a>(
        &'a self,
        board: &'a B,
        origin: B::Vector,
        step: B::Vector,
        check_colliders: impl Fn(B::Vector, TargetKind) -> bool + 'a,
    ) -> impl Iterator<Item = ScanTarget<B>> + 'a {
        // cannot do because return type doesn't match
        // if !(self.only_from_position.is_empty() || self.only_from_position.contains(&origin)) {
        //     return std::iter::empty();
        // }

        let board_iter = board.scan(origin, step);
        let board_iter = match &self.range {
            Some(range) => {
                itertools::Either::Left(board_iter.skip(range.min).take(range.max - range.min))
            }
            None => itertools::Either::Right(board_iter),
        };

        // TODO: how to avoid incorporating HashMap<Position, Piece>?
        // return impl Iterator<Item = impl Iterator>??
        // this runs into problems where each item is a different type
        // perhaps the consumer should execute collect_symmetries
        // MAYBE Pattern _only_ contains step, symmetries and exposes Pattern::collect_symmetries
        // then Scanner actually executes scanning?

        let mut scanned_positions = vec![];
        let mut collision_count = 0;
        board_iter
            .map(move |position| {
                if check_colliders(position, self.collisions.allowed_targets) {
                    collision_count += 1;
                }
                scanned_positions.push(position);
                (position, scanned_positions.clone(), collision_count)
            })
            // TODO: are these ordered correctly?
            .skip_while(|(_, _, collision_count)| *collision_count < self.collisions.min)
            .take_while(|(_, _, collision_count)| *collision_count < self.collisions.max)
            .map(|(position, scanned_positions, _)| ScanTarget {
                target: position,
                scanned_positions,
            })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Range {
    min: usize,
    max: usize,
}

impl Range {
    pub fn between(min: usize, max: usize) -> Self {
        Self { min, max }
    }

    pub fn up_to(max: usize) -> Self {
        Self::between(0, max)
    }

    pub fn starting_at(min: usize) -> Self {
        Self::between(min, 0)
    }
}

impl Default for Range {
    fn default() -> Self {
        Range::between(0, 1)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct PatternCollisions {
    min: u32,
    max: u32,
    allowed_targets: TargetKind,
}

impl PatternCollisions {
    pub fn with_min(mut self, min: u32) -> Self {
        self.min = min;
        if self.max < min {
            self.max = min;
        }
        self
    }

    pub fn with_max(mut self, max: u32) -> Self {
        self.max = max;
        if self.min > max {
            self.min = max;
        }
        self
    }

    pub fn for_targets(mut self, targets: TargetKind) -> Self {
        self.allowed_targets = targets;
        self
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub enum TargetKind {
    #[default]
    Any,
    Enemy,
    Friendly,
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct ScanTarget<B: GameBoard> {
    pub target: B::Vector,
    pub scanned_positions: Vec<B::Vector>,
}
