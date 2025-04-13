use crate::pieces::traits::{Movable, Piece};
use crate::pieces::types::{Color, MoveOffsets};

#[derive(Clone, Copy)]
pub struct Queen {
  color: Color,
}
const QUEEN_MOVES: [(i32, i32); 8] = [
  (1, 0),
  (-1, 0),
  (0, 1),
  (0, -1),
  (1, 1),
  (1, -1),
  (-1, 1),
  (-1, -1),
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
  fn get_move_offsets(&self) -> MoveOffsets {
    MoveOffsets::new_appliable_multiple(QUEEN_MOVES.to_vec())
  }

  fn is_movement_include_multible_steps(&self) -> bool {
    true
  }
}
