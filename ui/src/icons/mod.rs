use bevy::{
    prelude::{Changed, Commands, Component, Entity, Or, Query},
    utils::HashMap,
};

use egui_extras::RetainedImage;

use games::chess::{
    behavior::{PatternBehavior, RelayBehavior},
    pattern::Pattern,
    pieces::{PieceIdentity, Royal},
    team::Team,
};

mod classical;

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

    pub fn wild_svg(patterns: &Vec<Pattern>, team: Team, is_king: bool) -> Self {
        let (generated_icon, icon_source) = wild_behavior_icon(patterns, team, is_king);
        PieceIcon::svg(generated_icon, icon_source)
    }

    pub fn from_behaviors(
        patterns: Option<&PatternBehavior>,
        relays: Option<&RelayBehavior>,
        team: Team,
        is_royal: bool,
    ) -> Self {
        let patterns = patterns
            .map(|behavior| &behavior.patterns)
            .or(relays.map(|behavior| &behavior.patterns));
        PieceIcon::wild_svg(patterns.unwrap_or(&Vec::new()), team, is_royal)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct PieceIconHashKey<'a> {
    patterns: Option<&'a PatternBehavior>,
    relays: Option<&'a RelayBehavior>,
    team: &'a Team,
    is_king: bool,
}

#[allow(clippy::type_complexity)]
pub fn attach_piece_icons(
    mut commands: Commands,
    piece_query: Query<
        (
            Entity,
            &Team,
            &PieceIdentity,
            Option<&PatternBehavior>,
            Option<&RelayBehavior>,
            Option<&Royal>,
        ),
        Or<(Changed<PatternBehavior>, Changed<RelayBehavior>)>,
    >,
) {
    let mut icons = HashMap::<PieceIconHashKey, PieceIcon>::new();
    for (entity, team, identity, patterns, relays, maybe_royal) in piece_query.iter() {
        let key = PieceIconHashKey {
            patterns,
            relays,
            team,
            is_king: maybe_royal.is_some(),
        };
        let icon = if let Some(icon) = icons.get(&key) {
            Some(icon)
        } else if let PieceIdentity::Wild = identity {
            // otherwise create a new icon with the movement patterns
            // (or the relay patterns if no movement patterns exist)
            let icon = PieceIcon::from_behaviors(patterns, relays, *team, maybe_royal.is_some());
            icons.insert(key.clone(), icon.clone());
            icons.get(&key)
        } else {
            // if there is a known identity, use that as the icon
            icons.insert(
                key.clone(),
                PieceIcon::Character(classical::piece_unicode(identity, team)),
            );
            icons.get(&key)
        };
        if let Some(icon) = icon {
            commands.entity(entity).insert(icon.clone());
        }
    }
}
