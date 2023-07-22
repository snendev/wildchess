use bevy::{prelude::Component, utils::HashMap};

use crate::{Square, TargetMode};

#[derive(Clone, Component, Default, Debug)]
pub struct Vision {
    targets: HashMap<Square, TargetMode>,
}

impl Vision {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, targets: HashMap<Square, TargetMode>) {
        self.targets = targets;
    }

    pub fn can_target(&self, square: &Square) -> bool {
        self.targets.get(square).is_some()
    }

    pub fn can_attack(&self, square: &Square) -> bool {
        self.targets
            .get(square)
            .map(|target_mode| match target_mode {
                TargetMode::Attacking | TargetMode::OnlyAttacking => true,
                TargetMode::Moving => false,
            })
            .unwrap_or(false)
    }
}
