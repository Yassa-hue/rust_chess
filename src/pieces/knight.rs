use crate::pieces::traits::{Movable, Piece};
use crate::pieces::types::color::Color;
use crate::pieces::types::move_direction::{MoveDirection, MoveOffsets};
use crate::pieces::types::position::Position;

#[derive(Clone, Copy)]
pub struct Knight {
  color: Color,
}
const KNIGHT_MOVE_OFFSETS: [MoveDirection; 8] = [
  MoveDirection::KnightUpLeft,
  MoveDirection::KnightUpRight,
  MoveDirection::KnightDownLeft,
  MoveDirection::KnightDownRight,
  MoveDirection::KnightLeftUp,
  MoveDirection::KnightLeftDown,
  MoveDirection::KnightRightUp,
  MoveDirection::KnightRightDown,
];

impl Knight {
  pub fn new(color: Color) -> Self {
    Knight { color }
  }
}

impl Piece for Knight {
  fn color(&self) -> &Color {
    &self.color
  }
}

impl Movable for Knight {
  fn get_move_offsets(&self, _: Position) -> MoveOffsets {
    MoveOffsets::new_appliable_once(KNIGHT_MOVE_OFFSETS.to_vec())
  }
}
