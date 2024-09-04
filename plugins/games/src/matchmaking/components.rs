use layouts::{FeaturedWildLayout, RandomWildLayout};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use bevy_ecs::prelude::{Bundle, Component};
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::Reflect;

use crate::components::{Clock, PieceSet};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub enum GameRequestVariant {
    #[default]
    FeaturedGameOne,
    FeaturedGameTwo,
    FeaturedGameThree,
    Wild,
    // TODO: configuration...?
}

impl GameRequestVariant {
    pub fn piece_set(&self) -> PieceSet {
        PieceSet(match self {
            GameRequestVariant::FeaturedGameOne => FeaturedWildLayout::One.pieces(),
            GameRequestVariant::FeaturedGameTwo => FeaturedWildLayout::Two.pieces(),
            GameRequestVariant::FeaturedGameThree => FeaturedWildLayout::Three.pieces(),
            GameRequestVariant::Wild => RandomWildLayout::pieces(),
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub enum GameRequestClock {
    #[default]
    Bullet,
    Blitz,
    Rapid,
    Classical,
}

impl GameRequestClock {
    pub fn to_clock(self) -> Clock {
        match self {
            GameRequestClock::Bullet => {
                Clock::new(Duration::from_secs(120), Duration::from_secs(0))
            }
            GameRequestClock::Blitz => Clock::new(Duration::from_secs(300), Duration::from_secs(1)),
            GameRequestClock::Rapid => Clock::new(Duration::from_secs(600), Duration::from_secs(5)),
            GameRequestClock::Classical => {
                Clock::new(Duration::from_secs(3600), Duration::from_secs(30))
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct GameRequest;

#[derive(Clone, Debug)]
#[derive(Bundle)]
#[derive(Deserialize, Serialize)]
pub struct GameRequestBundle {
    pub request: GameRequest,
    pub variant: GameRequestVariant,
    pub clock: GameRequestClock,
}
