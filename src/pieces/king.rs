use crate::pieces::traits::Movable;
use crate::pieces::types::{Color, MoveOffsets};

#[derive(Clone, Copy)]
pub struct King {
  color: Color,
}
const KING_MOVES: [(i32, i32); 8] = [
  (1, 0),
  (-1, 0),
  (0, 1),
  (0, -1),
  (1, 1),
  (1, -1),
  (-1, 1),
  (-1, -1),
];

impl King {
  pub fn new(color: Color) -> Self {
    King { color }
  }
}

impl Movable for King {
  fn get_move_offsets(&self) -> MoveOffsets {
    MoveOffsets::new_appliable_once(KING_MOVES.to_vec())
  }

  fn color(&self) -> &Color {
    &self.color
  }
}
