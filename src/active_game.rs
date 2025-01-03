use bevy::{
    log,
    prelude::{Commands, Component, Entity, Query, With, Without},
};
use games::chess::board::Board;

impl Active {
    pub(crate) fn set_active(
        mut commands: Commands,
        inactive_boards: Query<Entity, (With<Board>, Without<Active>)>,
        active_boards: Query<Entity, (With<Board>, With<Active>)>,
    ) {
        log::info!("set_active running");
        if !active_boards.is_empty() {
            return;
        }
        if let Some(board) = inactive_boards.iter().next() {
            log::info!("Setting active board: {board}");
            commands.entity(board).insert(Active);
        }
    }
}

#[derive(Debug)]
#[derive(Component)]
pub struct Active;
