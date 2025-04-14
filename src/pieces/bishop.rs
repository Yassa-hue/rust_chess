use crate::pieces::traits::{Movable, Piece};
use crate::pieces::types::color::Color;
use crate::pieces::types::move_direction::{MoveDirection, MoveOffsets};
use crate::pieces::types::position::Position;

#[derive(Clone, Copy)]
pub struct Bishop {
  color: Color,
}
const BISHOP_MOVES: [MoveDirection; 4] = [
  MoveDirection::UpRight,
  MoveDirection::UpLeft,
  MoveDirection::DownRight,
  MoveDirection::DownLeft,
];

impl Bishop {
  pub fn new(color: Color) -> Self {
    Bishop { color }
  }
}

impl Piece for Bishop {
  fn color(&self) -> &Color {
    &self.color
  }
}

impl Movable for Bishop {
  fn get_move_offsets(&self, _: Position) -> MoveOffsets {
    MoveOffsets::new_appliable_multiple(BISHOP_MOVES.to_vec())
  }
}
