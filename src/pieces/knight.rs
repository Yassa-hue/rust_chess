use crate::pieces::traits::{Movable, Piece};
use crate::pieces::types::color::Color;
use crate::pieces::types::move_direction::{Direction, MovementPattern};
use crate::pieces::types::position::Position;

#[derive(Clone, Copy)]
pub struct Knight {
  color: Color,
}

const KNIGHT_MOVES: [Direction; 8] = [
  Direction::KnightUpLeft,
  Direction::KnightUpRight,
  Direction::KnightDownLeft,
  Direction::KnightDownRight,
  Direction::KnightLeftUp,
  Direction::KnightLeftDown,
  Direction::KnightRightUp,
  Direction::KnightRightDown,
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
  fn movement_pattern(&self, _: Position) -> MovementPattern {
    MovementPattern::new_appliable_once(KNIGHT_MOVES.to_vec())
  }
}
