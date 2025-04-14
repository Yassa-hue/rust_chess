use crate::pieces::traits::{Movable, Piece};
use crate::pieces::types::{Color, MoveOffsets, Position};

const PAWN_X_START_POSITIONS: [usize; 2] = [
  1, // White
  6, // Black
];

#[derive(Clone, Copy)]
pub struct Pawn {
  color: Color,
}

impl Pawn {
  pub fn new(color: Color) -> Self {
    Pawn { color }
  }
}

impl Piece for Pawn {
  fn color(&self) -> &Color {
    &self.color
  }
}

impl Movable for Pawn {
  fn get_move_offsets(&self, current_position: Position) -> MoveOffsets {
    let offsets = match self.color {
      Color::White => vec![(1, 0)],
      Color::Black => vec![(-1, 0)],
    };

    // Pawns can move two squares forward from their starting position
    if PAWN_X_START_POSITIONS.iter().any(|x| *x == current_position.x()) {
      MoveOffsets::new_appliable_twice(offsets)
    } else {
      // Pawns can only move one square forward otherwise
      MoveOffsets::new_appliable_once(offsets)
    }
  }
}
