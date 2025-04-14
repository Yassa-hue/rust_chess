pub mod traits;
pub mod types;

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

pub use bishop::Bishop;
pub use king::King;
pub use knight::Knight;
pub use pawn::Pawn;
pub use queen::Queen;
pub use rook::Rook;
pub use traits::*;
#[allow(unused_imports)] // for testing purposes
pub use types::*;

#[cfg(test)]
pub mod tests;
