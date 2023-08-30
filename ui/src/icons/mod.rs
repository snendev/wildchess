use bevy::{
    prelude::{Changed, Commands, Component, Entity, Query},
    utils::HashMap,
};

use egui_extras::RetainedImage;

use chess_gameplay::chess::{
    pieces::{Behavior, King},
    team::Team,
};

// mod classical;

mod wild;
use wild::wild_behavior_icon;

#[derive(Clone, Component)]
pub enum PieceIcon {
    Svg {
        image: std::sync::Arc<RetainedImage>,
        source: String,
    },
    Character(char),
}

// enforces an override icon in case the generated icons should not be used
#[derive(Component)]
pub struct IconOverride(PieceIcon);

impl PieceIcon {
    pub fn svg(image: RetainedImage, source: String) -> Self {
        PieceIcon::Svg {
            image: std::sync::Arc::new(image),
            source,
        }
    }

    pub fn character(character: char) -> Self {
        PieceIcon::Character(character)
    }

    pub fn wild_svg(behavior: &Behavior, team: Team, is_king: bool) -> Self {
        let (generated_icon, icon_source) = wild_behavior_icon(behavior, team, is_king);
        PieceIcon::svg(generated_icon, icon_source)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct PieceIconHashKey<'a> {
    behavior: &'a Behavior,
    team: &'a Team,
    is_king: bool,
}

pub fn attach_piece_icons(
    mut commands: Commands,
    piece_query: Query<
        (
            Entity,
            &Behavior,
            &Team,
            Option<&King>,
            Option<&IconOverride>,
        ),
        Changed<Behavior>,
    >,
) {
    let mut icons = HashMap::<PieceIconHashKey, PieceIcon>::new();
    for (entity, behavior, team, king, override_icon) in piece_query.iter() {
        let key = PieceIconHashKey {
            behavior,
            team,
            is_king: king.is_some(),
        };
        let icon = if let Some(icon) = icons.get(&key) {
            Some(icon)
        } else if let Some(override_icon) = override_icon {
            icons.insert(key.clone(), override_icon.0.clone());
            icons.get(&key)
        } else {
            // default to creating a new icon
            let icon = PieceIcon::wild_svg(behavior, *team, king.is_some());
            icons.insert(key.clone(), icon.clone());
            icons.get(&key)
        };
        if let Some(icon) = icon {
            commands.entity(entity).insert(icon.clone());
        }
    }
}
