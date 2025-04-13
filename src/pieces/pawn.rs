use crate::pieces::traits::{Movable, Piece};
use crate::pieces::types::{Color, MoveOffsets};

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
  fn get_move_offsets(&self) -> MoveOffsets {
    let offsets = match self.color {
      Color::White => vec![(1, 0)],
      Color::Black => vec![(-1, 0)],
    };
    MoveOffsets::new_appliable_once(offsets)
  }

  fn is_movement_include_multible_steps(&self) -> bool {
    false
  }
}
