use crate::pieces::traits::{Movable, Piece};
use crate::pieces::types::color::Color;
use crate::pieces::types::move_direction::{MoveDirection, MoveOffsets};
use crate::pieces::types::position::Position;

#[derive(Clone, Copy)]
pub struct Rook {
  color: Color,
}

const ROOK_MOVES: [MoveDirection; 4] = [
  MoveDirection::Up,
  MoveDirection::Down,
  MoveDirection::Left,
  MoveDirection::Right,
];

impl Rook {
  pub fn new(color: Color) -> Self {
    Rook { color }
  }
}

impl Piece for Rook {
  fn color(&self) -> &Color {
    &self.color
  }
}

impl Movable for Rook {
  fn get_move_offsets(&self, _: Position) -> MoveOffsets {
    MoveOffsets::new_appliable_multiple(ROOK_MOVES.to_vec())
  }
}
