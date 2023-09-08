use bevy::{
    prelude::{Component, Reflect, ReflectComponent},
    utils::HashMap,
};

use crate::{pieces::TargetMode, square::Square};

#[derive(Clone, Component, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Targets(HashMap<Square, TargetMode>);

impl Targets {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, targets: HashMap<Square, TargetMode>) {
        self.0 = targets;
    }

    pub fn can_target(&self, square: &Square) -> bool {
        self.0.get(square).is_some()
    }

    pub fn can_attack(&self, square: &Square) -> bool {
        self.0
            .get(square)
            .map(|target_mode| match target_mode {
                TargetMode::Attacking | TargetMode::OnlyAttacking => true,
                TargetMode::Moving => false,
            })
            .unwrap_or(false)
    }
}
