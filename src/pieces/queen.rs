use crate::pieces::traits::{Movable, Piece};
use crate::pieces::types::color::Color;
use crate::pieces::types::move_direction::{MoveDirection, MoveOffsets};
use crate::pieces::types::position::Position;

#[derive(Clone, Copy)]
pub struct Queen {
  color: Color,
}
const QUEEN_MOVES: [MoveDirection; 8] = [
  MoveDirection::Up,
  MoveDirection::Down,
  MoveDirection::Left,
  MoveDirection::Right,
  MoveDirection::UpLeft,
  MoveDirection::UpRight,
  MoveDirection::DownLeft,
  MoveDirection::DownRight,
];

impl Queen {
  pub fn new(color: Color) -> Self {
    Queen { color }
  }
}

impl Piece for Queen {
  fn color(&self) -> &Color {
    &self.color
  }
}

impl Movable for Queen {
  fn get_move_offsets(&self, _: Position) -> MoveOffsets {
    MoveOffsets::new_appliable_multiple(QUEEN_MOVES.to_vec())
  }
}
