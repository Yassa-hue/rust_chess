use crate::pieces::traits::{Movable, Piece};
use crate::pieces::types::color::Color;
use crate::pieces::types::move_direction::{MoveDirection, MoveOffsets};
use crate::pieces::types::position::Position;

#[derive(Clone, Copy)]
pub struct King {
  color: Color,
}
const KING_MOVES: [MoveDirection; 8] = [
  MoveDirection::Up,
  MoveDirection::Down,
  MoveDirection::Left,
  MoveDirection::Right,
  MoveDirection::UpLeft,
  MoveDirection::UpRight,
  MoveDirection::DownLeft,
  MoveDirection::DownRight,
];

impl King {
  pub fn new(color: Color) -> Self {
    King { color }
  }
}

impl Piece for King {
  fn color(&self) -> &Color {
    &self.color
  }
}

impl Movable for King {
  fn get_move_offsets(&self, _: Position) -> MoveOffsets {
    MoveOffsets::new_appliable_once(KING_MOVES.to_vec())
  }
}
