use bevy::{
    prelude::{Component, Reflect, ReflectComponent},
    utils::{HashMap, HashSet},
};

use crate::{
    square::{Rank, Square},
    team::Team,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub enum TargetMode {
    #[default]
    Attacking,
    Moving,
    OnlyAttacking,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub enum SearchMode {
    #[default]
    Walk,
    Jump,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct PatternStep {
    // all x are symmetric; if it can move right, it can move left
    pub x: u8,
    // y needs specifying whether forward or backward
    pub y: i16,
}

impl PatternStep {
    pub fn look(x: u8, y: i16) -> Self {
        PatternStep { x, y }
    }

    pub fn forward() -> Self {
        PatternStep::look(0, 1)
    }

    pub fn sideways() -> Self {
        PatternStep::look(1, 0)
    }

    pub fn backward() -> Self {
        PatternStep::look(0, -1)
    }

    pub fn diagonal_forward() -> Self {
        PatternStep::look(1, 1)
    }

    pub fn diagonal_backward() -> Self {
        PatternStep::look(1, -1)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum OriginConstraint {
    LocalRank(Rank),
}

// The calculation type for board searches
#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct Pattern {
    // the unit of "stepping" for searching the board
    pub step: PatternStep,
    // how many steps can this pattern be executed for?
    // if None, do not set a limit
    pub range: Option<u8>,
    // this determines whether capture is allowed along this vector
    pub target_mode: TargetMode,
    // whether to stop search when colliding with another piece
    pub search_mode: SearchMode,
    // which squares this pattern can be activated from, if any
    pub origin_constrant: Option<OriginConstraint>,
}

impl Pattern {
    pub fn new(step: PatternStep) -> Self {
        Pattern {
            step,
            range: None,
            target_mode: TargetMode::Moving,
            search_mode: SearchMode::Walk,
            origin_constrant: None,
        }
    }

    pub fn look(x: u8, y: i16) -> Self {
        Pattern::new(PatternStep::look(x, y))
    }

    pub fn forward() -> Self {
        Pattern::new(PatternStep::forward())
    }

    pub fn sideways() -> Self {
        Pattern::new(PatternStep::sideways())
    }

    pub fn backward() -> Self {
        Pattern::new(PatternStep::backward())
    }

    pub fn diagonal_forward() -> Self {
        Pattern::new(PatternStep::diagonal_forward())
    }

    pub fn diagonal_backward() -> Self {
        Pattern::new(PatternStep::diagonal_backward())
    }

    pub fn range(mut self, range: u8) -> Self {
        self.range = Some(range);
        self
    }

    pub fn cannot_attack(mut self) -> Self {
        self.target_mode = TargetMode::Moving;
        self
    }

    pub fn can_attack(mut self) -> Self {
        self.target_mode = TargetMode::Attacking;
        self
    }

    pub fn must_attack(mut self) -> Self {
        self.target_mode = TargetMode::OnlyAttacking;
        self
    }

    pub fn walking(mut self) -> Self {
        self.search_mode = SearchMode::Walk;
        self
    }

    pub fn jumping(mut self) -> Self {
        self.search_mode = SearchMode::Jump;
        self
    }

    pub fn only_from_local_rank(mut self, rank: Rank) -> Self {
        self.origin_constrant = Some(OriginConstraint::LocalRank(rank));
        self
    }

    fn get_movement_steps(&self, team: Team) -> Vec<(i16, i16)> {
        let x = self.step.x.into();
        let y = self.step.y;

        match (team, x) {
            (Team::White, 0) => vec![(x, y)],
            (Team::Black, 0) => vec![(x, -y)],
            (Team::White, x) => vec![(x, y), (-x, y)],
            (Team::Black, x) => vec![(x, -y), (-x, -y)],
        }
    }
}

// A type that helps build collections of Patterns
pub struct BehaviorBuilder {
    steps: Vec<PatternStep>,
    range: Option<u8>,
    target_mode: TargetMode,
    search_mode: SearchMode,
}

impl Default for BehaviorBuilder {
    fn default() -> Self {
        BehaviorBuilder {
            steps: vec![],
            range: None,
            target_mode: TargetMode::Moving,
            search_mode: SearchMode::Walk,
        }
    }
}

impl BehaviorBuilder {
    pub fn steps(steps: Vec<PatternStep>) -> Self {
        BehaviorBuilder {
            steps,
            ..Default::default()
        }
    }

    pub fn forward(mut self) -> Self {
        self.steps.push(PatternStep::forward());
        self
    }

    pub fn sideways(mut self) -> Self {
        self.steps.push(PatternStep::sideways());
        self
    }

    pub fn backward(mut self) -> Self {
        self.steps.push(PatternStep::backward());
        self
    }

    pub fn diagonal_forward(mut self) -> Self {
        self.steps.push(PatternStep::diagonal_forward());
        self
    }

    pub fn diagonal_backward(mut self) -> Self {
        self.steps.push(PatternStep::diagonal_backward());
        self
    }

    pub fn diagonals(mut self) -> Self {
        let mut diagonals = vec![
            PatternStep::diagonal_forward(),
            PatternStep::diagonal_backward(),
        ];
        self.steps.append(&mut diagonals);
        self
    }

    pub fn orthogonals(mut self) -> Self {
        let mut orthogonals = vec![
            PatternStep::forward(),
            PatternStep::sideways(),
            PatternStep::backward(),
        ];
        self.steps.append(&mut orthogonals);
        self
    }

    pub fn radials(self) -> Self {
        self.orthogonals().diagonals()
    }

    pub fn knight_jumps(mut self) -> Self {
        let mut steps = vec![
            PatternStep::look(1, 2),
            PatternStep::look(2, 1),
            PatternStep::look(1, -2),
            PatternStep::look(2, -1),
        ];
        self.steps.append(&mut steps);
        self
    }

    pub fn range(mut self, range: u8) -> Self {
        self.range = Some(range);
        self
    }

    pub fn cannot_attack(mut self) -> Self {
        self.target_mode = TargetMode::Moving;
        self
    }

    pub fn can_attack(mut self) -> Self {
        self.target_mode = TargetMode::Attacking;
        self
    }

    pub fn must_attack(mut self) -> Self {
        self.target_mode = TargetMode::OnlyAttacking;
        self
    }

    pub fn walking(mut self) -> Self {
        self.search_mode = SearchMode::Walk;
        self
    }

    pub fn jumping(mut self) -> Self {
        self.search_mode = SearchMode::Jump;
        self
    }

    pub fn build(self) -> Behavior {
        self.steps
            .into_iter()
            .map(|step| Pattern {
                step,
                range: self.range,
                target_mode: self.target_mode,
                search_mode: self.search_mode,
                origin_constrant: None,
            })
            .collect::<Vec<_>>()
            .into()
    }
}

#[derive(Clone, Debug, Default, Component, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct Behavior {
    pub patterns: Vec<Pattern>,
}

impl From<Vec<Pattern>> for Behavior {
    fn from(patterns: Vec<Pattern>) -> Self {
        Behavior { patterns }
    }
}

impl Behavior {
    pub fn builder() -> BehaviorBuilder {
        BehaviorBuilder::default()
    }

    pub fn join(mut self, mut other: Self) -> Self {
        self.patterns.append(&mut other.patterns);
        self
    }

    pub fn with_pattern(mut self, pattern: Pattern) -> Self {
        self.patterns.push(pattern);
        self
    }
}

// From here, we can define how board search is performed

// Each Pattern can perform its own search and yield a set of squares
impl Pattern {
    pub fn search<T>(
        &self,
        origin: &Square,
        my_team: Team,
        pieces: &HashMap<Square, (T, Team)>,
    ) -> HashSet<Square> {
        let mut squares: HashSet<Square> = HashSet::new();
        for (x, y) in self.get_movement_steps(my_team) {
            let mut current_square = *origin;
            let mut current_range = 0;
            let only_allow_capture = match self.target_mode {
                TargetMode::Moving => false,
                TargetMode::Attacking => false,
                TargetMode::OnlyAttacking => true,
            };
            let allow_capture = match self.target_mode {
                TargetMode::Moving => false,
                TargetMode::Attacking => true,
                TargetMode::OnlyAttacking => true,
            };
            while let Some(square) = current_square.checked_add(y, x) {
                current_range += 1;
                if let Some(range) = self.range {
                    if current_range > range {
                        break;
                    }
                }

                if let Some((_, team)) = pieces.get(&square) {
                    if my_team != *team && allow_capture {
                        squares.insert(square);
                    }
                    match self.search_mode {
                        SearchMode::Walk => break,
                        SearchMode::Jump => {
                            current_square = square;
                            continue;
                        }
                    }
                } else {
                    if !only_allow_capture {
                        squares.insert(square);
                    }
                    current_square = square;
                }
            }
        }
        squares
    }
}

// When a Behavior runs a search, it must return a struct that contains
// the TargetMode (for visualization purposes)
impl Behavior {
    pub fn search<T>(
        &self,
        origin: &Square,
        my_team: Team,
        pieces: &HashMap<Square, (T, Team)>,
    ) -> HashMap<Square, TargetMode> {
        let mut targets: HashMap<Square, TargetMode> = HashMap::new();
        for (squares, pattern) in self
            .patterns
            .iter()
            .map(|pattern| (pattern.search(origin, my_team, pieces), pattern))
        {
            for square in squares {
                if let Some(mode) = targets.get(&square) {
                    match (mode, pattern.target_mode) {
                        // upgrade the target mode if appropriate
                        (TargetMode::OnlyAttacking, TargetMode::Moving)
                        | (TargetMode::Moving, TargetMode::OnlyAttacking)
                        | (_, TargetMode::Attacking) => {
                            targets.insert(square, TargetMode::Attacking);
                        }
                        _ => {}
                    }
                } else {
                    targets.insert(square, pattern.target_mode);
                }
            }
        }
        targets
    }
}
