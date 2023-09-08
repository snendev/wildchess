pub mod classical;
pub mod wild;

mod plugin;
pub use plugin::{BoardPlugin, BuildClassicalBoardEvent, BuildWildBoardEvent};

pub(crate) mod utils;
