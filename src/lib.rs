//! `vault` is a Company of Heroes 3 replay parser written with nom, and exposing Ruby bindings
//! via magnus.
//!
//! Currently, this library is able to parse player, map, and chat message data, as well as various
//! bits and pieces of basic replay information. It is not currently able to parse detailed command
//! information
//!
//! To use, simply initialize an instance of `Replay` by using one of its parsing entrypoints:
//!
//! ```ignore
//! fn main() {
//!     let data = include_bytes!("/path/to/replay.rec");
//!     let replay = vault::Replay::from_bytes(data);
//!     assert!(replay.is_ok())
//! }
//! ```

pub mod commands;
mod data;
mod errors;
mod map;
mod message;
mod player;
mod replay;

pub use crate::commands::Command;
pub use crate::errors::ParseError;
pub use crate::map::Map;
pub use crate::message::Message;
pub use crate::player::Faction;
pub use crate::player::Player;
pub use crate::player::Team;
pub use crate::replay::Replay;
pub use crate::data::commands::Raw;
