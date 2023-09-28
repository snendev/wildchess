// TODO:
// mod castling;
// mod mirror;
// mod rotation;

mod pattern;
pub use pattern::PatternBehavior;

mod en_passant;
pub use en_passant::EnPassantBehavior;

mod mimic;
pub use mimic::MimicBehavior;

mod relay;
pub use relay::RelayBehavior;
