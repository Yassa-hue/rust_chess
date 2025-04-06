use crate::pieces::traits::Movable;
use crate::pieces::types::{Color, MoveOffsets};

#[derive(Clone, Copy)]
pub struct Rook {
  color: Color,
}
const ROOK_MOVES: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
impl Rook {
  pub fn new(color: Color) -> Self {
    Rook { color }
  }
}

impl Movable for Rook {
  fn get_move_offsets(&self) -> MoveOffsets {
    MoveOffsets::new_appliable_multiple(ROOK_MOVES.to_vec())
  }

  fn color(&self) -> &Color {
    &self.color
  }
}
