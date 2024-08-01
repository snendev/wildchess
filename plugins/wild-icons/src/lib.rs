use std::marker::PhantomData;

use bevy::{
    prelude::{
        App, Changed, Commands, Component, Entity, In, IntoSystem, Local, Or, Plugin, PreUpdate,
        Query,
    },
    utils::HashMap,
};

use games::chess::{
    behavior::{PatternBehavior, RelayBehavior},
    pieces::{Orientation, PieceIdentity, Royal},
    team::Team,
};

mod classical;

mod wild;
use wild::wild_behavior_icon;

#[derive(Clone)]
#[derive(Component)]
pub struct PieceIconSvg {
    pub source: String,
    pub bytes: Vec<u8>,
    pub uri: String,
    pub label: String,
}

#[derive(Clone)]
#[derive(Component)]
pub struct PieceIconCharacter {
    pub character: char,
}

impl PieceIconSvg {
    pub fn new(
        identity: PieceIdentity,
        key: impl Into<String>,
        patterns: Option<&PatternBehavior>,
        relays: Option<&RelayBehavior>,
        team: Team,
        board_orientation: Orientation,
        is_royal: bool,
    ) -> Self {
        // let image = ImageSource::Bytes {
        //     uri: format!("bytes://{}.svg", label).into(),
        //     bytes: source.bytes().collect::<Vec<u8>>().into(),
        // };
        let patterns = patterns
            .map(|behavior| &behavior.patterns)
            .or(relays.map(|behavior| &behavior.patterns));
        let icon_source = wild_behavior_icon(
            patterns.unwrap_or(&vec![]),
            team,
            board_orientation,
            is_royal,
        );
        let label = format!("{:?}-{}", identity, key.into());
        PieceIconSvg {
            // image,
            bytes: icon_source.bytes().collect::<Vec<u8>>(),
            source: icon_source,
            uri: format!("bytes://{}.svg", label),
            label,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PieceIconKey {
    patterns: Option<PatternBehavior>,
    relays: Option<RelayBehavior>,
    team: Team,
    is_king: bool,
}

enum PieceIcon {
    Svg(PieceIconSvg),
    Character(PieceIconCharacter),
}

pub struct PieceIconPlugin<System, Params>
where
    System: IntoSystem<(), Orientation, Params>,
{
    get_orientation: System,
    marker: PhantomData<Params>,
}

impl<System, Params> PieceIconPlugin<System, Params>
where
    System: IntoSystem<(), Orientation, Params>,
{
    pub fn new(get_orientation: System) -> Self {
        Self {
            get_orientation,
            marker: PhantomData::<Params>,
        }
    }
}

impl<System, Params> Plugin for PieceIconPlugin<System, Params>
where
    System: IntoSystem<(), Orientation, Params> + Clone + Send + Sync + 'static,
    Params: Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            self.get_orientation.clone().pipe(Self::attach_icons_system),
        );
    }
}

impl<System, Params> PieceIconPlugin<System, Params>
where
    System: IntoSystem<(), Orientation, Params>,
{
    #[allow(clippy::type_complexity)]
    fn attach_icons_system(
        In(board_orientation): In<Orientation>,
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
        mut icons: Local<HashMap<PieceIconKey, PieceIcon>>,
    ) {
        for (entity, team, identity, patterns, relays, maybe_royal) in piece_query.iter() {
            let key = PieceIconKey {
                patterns: patterns.cloned(),
                relays: relays.cloned(),
                team: *team,
                is_king: maybe_royal.is_some(),
            };
            let icon = if let Some(icon) = icons.get(&key) {
                Some(icon)
            } else {
                // otherwise create a new icon with the movement patterns
                // (or the relay patterns if no movement patterns exist)
                let icon = PieceIconSvg::new(
                    *identity,
                    format!("{:?}", key),
                    patterns,
                    relays,
                    *team,
                    board_orientation,
                    maybe_royal.is_some(),
                );
                icons.insert(key.clone(), PieceIcon::Svg(icon.clone()));
                icons.get(&key)
            }
            // else {
            //     // if there is a known identity, use that as the icon
            //     icons.insert(
            //         key.clone(),
            //         PieceIcon::Character(PieceIconCharacter {
            //             character: classical::piece_unicode(identity, team),
            //         }),
            //     );
            //     icons.get(&key)
            // }
            ;
            if let Some(icon) = icon {
                match icon {
                    PieceIcon::Svg(icon) => commands.entity(entity).insert(icon.clone()),
                    PieceIcon::Character(icon) => commands.entity(entity).insert(icon.clone()),
                };
            }
        }
    }
}
