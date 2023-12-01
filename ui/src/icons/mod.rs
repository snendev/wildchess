use bevy::{
    prelude::{Changed, Commands, Component, Entity, Or, Query},
    utils::HashMap,
};

use bevy_egui::egui::ImageSource;

use games::chess::{
    behavior::{PatternBehavior, RelayBehavior},
    pieces::{PieceIdentity, Royal},
    team::Team,
};

mod classical;

mod wild;
use wild::wild_behavior_icon;

#[derive(Clone, Component)]
pub enum PieceIcon<'a> {
    Svg {
        image: ImageSource<'a>,
        source: String,
    },
    Character(char),
}

impl<'a> PieceIcon<'a> {
    pub fn svg(label: String, source: String) -> Self {
        let image = ImageSource::Bytes {
            uri: format!("bytes://{}.svg", label).into(),
            bytes: source.bytes().collect::<Vec<u8>>().into(),
        };
        PieceIcon::Svg { image, source }
    }

    pub fn character(character: char) -> Self {
        PieceIcon::Character(character)
    }

    pub fn from_behaviors(
        identity: PieceIdentity,
        key: impl Into<String>,
        patterns: Option<&PatternBehavior>,
        relays: Option<&RelayBehavior>,
        team: Team,
        is_royal: bool,
    ) -> Self {
        let patterns = patterns
            .map(|behavior| &behavior.patterns)
            .or(relays.map(|behavior| &behavior.patterns));
        let icon_source = wild_behavior_icon(patterns.unwrap_or(&vec![]), team, is_royal);
        PieceIcon::svg(format!("{:?}-{}", identity, key.into()), icon_source)
    }

    #[allow(clippy::type_complexity)]
    pub fn attach_icons_system(
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
                let icon = PieceIcon::from_behaviors(
                    *identity,
                    format!("{:?}", key),
                    patterns,
                    relays,
                    *team,
                    maybe_royal.is_some(),
                );
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
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct PieceIconHashKey<'a> {
    patterns: Option<&'a PatternBehavior>,
    relays: Option<&'a RelayBehavior>,
    team: &'a Team,
    is_king: bool,
}
