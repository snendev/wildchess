// N.B. This is not _exactly_ Knight Relay Chess
// TODO: make knights Iron, remove knight capture
// Additionally, we do not have the constraint that pawns cannot use relayed knight powers to
// reach the last or first ranks.

mod layout;
pub(crate) use layout::KnightRelayLayout;
