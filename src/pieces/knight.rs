use crate::pieces::traits::Movable;
use crate::pieces::types::{Color, MoveOffsets};

#[derive(Clone, Copy)]
pub struct Knight {
  color: Color,
}
const KNIGHT_MOVE_OFFSETS: [(i32, i32); 8] = [
  (2, 1),
  (2, -1),
  (-2, 1),
  (-2, -1),
  (1, 2),
  (1, -2),
  (-1, 2),
  (-1, -2),
];

impl Knight {
  pub fn new(color: Color) -> Self {
    Knight { color }
  }
}

impl Movable for Knight {
  fn get_move_offsets(&self) -> MoveOffsets {
    MoveOffsets::new_appliable_once(KNIGHT_MOVE_OFFSETS.to_vec())
  }

  fn color(&self) -> &Color {
    &self.color
  }
}
