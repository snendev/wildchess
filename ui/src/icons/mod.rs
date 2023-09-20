use bevy::{
    prelude::{Changed, Commands, Component, Entity, Or, Query},
    utils::HashMap,
};

use chess_boards::classical::ClassicalIdentity;
use egui_extras::RetainedImage;

use chess_gameplay::chess::{
    behavior::{PatternBehavior, RelayBehavior},
    pattern::Pattern,
    pieces::Royal,
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
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct PieceIconHashKey<'a> {
    patterns: Option<&'a PatternBehavior>,
    relays: Option<&'a RelayBehavior>,
    team: &'a Team,
    is_king: bool,
}

pub fn attach_piece_icons(
    mut commands: Commands,
    piece_query: Query<
        (
            Entity,
            &Team,
            Option<&PatternBehavior>,
            Option<&RelayBehavior>,
            Option<&Royal>,
            Option<&ClassicalIdentity>,
        ),
        Or<(Changed<PatternBehavior>, Changed<RelayBehavior>)>,
    >,
) {
    let mut icons = HashMap::<PieceIconHashKey, PieceIcon>::new();
    for (entity, team, patterns, relays, maybe_royal, classical_identity) in piece_query.iter() {
        let key = PieceIconHashKey {
            patterns,
            relays,
            team,
            is_king: maybe_royal.is_some(),
        };
        let icon = if let Some(icon) = icons.get(&key) {
            Some(icon)
        } else if let Some(id) = classical_identity {
            // if there is a known identity, use that as the icon
            icons.insert(
                key.clone(),
                PieceIcon::Character(classical::piece_unicode(id, team)),
            );
            icons.get(&key)
        } else {
            // otherwise create a new icon with the movement patterns
            // (or the relay patterns if no movement patterns exist)
            // TODO: don't construct this unnecessarily
            let empty = Vec::new();
            let patterns = if let Some(patterns) = patterns {
                &patterns.patterns
            } else if let Some(relays) = relays {
                &relays.patterns
            } else {
                &empty
            };
            let icon = PieceIcon::wild_svg(patterns, *team, maybe_royal.is_some());
            icons.insert(key.clone(), icon.clone());
            icons.get(&key)
        };
        if let Some(icon) = icon {
            commands.entity(entity).insert(icon.clone());
        }
    }
}
