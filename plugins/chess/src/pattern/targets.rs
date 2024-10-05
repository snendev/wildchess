use serde::{Deserialize, Serialize};

use bevy::prelude::Reflect;

use crate::team::Team;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[derive(Reflect)]
#[derive(Deserialize, Serialize)]
pub enum TargetKind {
    #[default]
    Enemy,
    Friendly,
    Any,
}

impl TargetKind {
    pub fn matches(&self, my_team: &Team, target_team: &Team) -> bool {
        match self {
            TargetKind::Enemy => my_team != target_team,
            TargetKind::Friendly => my_team == target_team,
            TargetKind::Any => true,
        }
    }
}
