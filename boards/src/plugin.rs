use bevy::prelude::{
    on_event, App, Commands, Event, EventReader, IntoSystemConfigs, Plugin, Update,
};

use crate::{
    classical::{ClassicalIdentity, ClassicalLayout},
    wild::WildLayout,
};

#[derive(Clone, Copy, Debug, Default, Event)]
pub struct BuildWildBoardEvent;

#[derive(Clone, Copy, Debug, Default, Event)]
pub struct BuildClassicalBoardEvent;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BuildWildBoardEvent>()
            .add_event::<BuildClassicalBoardEvent>()
            .add_systems(
                Update,
                (
                    configure_wild_boards.run_if(on_event::<BuildWildBoardEvent>()),
                    configure_classical_boards.run_if(on_event::<BuildClassicalBoardEvent>()),
                ),
            );

        #[cfg(debug_assertions)]
        app.register_type::<ClassicalIdentity>();
    }
}

fn configure_wild_boards(mut commands: Commands, mut reader: EventReader<BuildWildBoardEvent>) {
    for _ in reader.iter() {
        for piece in WildLayout::pieces() {
            piece.spawn(&mut commands);
        }
    }
}

fn configure_classical_boards(
    mut commands: Commands,
    mut reader: EventReader<BuildClassicalBoardEvent>,
) {
    for _ in reader.iter() {
        for piece in ClassicalLayout::pieces() {
            piece.spawn(&mut commands);
        }
    }
}
