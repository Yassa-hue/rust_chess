use crate::pieces::traits::Movable;
use crate::pieces::types::color::Color;
use crate::pieces::types::move_direction::{Direction, MovementPattern};
use crate::pieces::types::position::Position;

#[derive(Clone, Copy)]
pub struct Rook {
  color: Color,
}

const ROOK_MOVES: [Direction; 4] = [
  Direction::Up,
  Direction::Down,
  Direction::Left,
  Direction::Right,
];

impl Rook {
  pub fn new(color: Color) -> Self {
    Rook { color }
  }
}

impl Rook {
  pub fn color(&self) -> &Color {
    &self.color
  }
}

impl Movable for Rook {
  fn movement_pattern(&self, _: Position) -> MovementPattern {
    MovementPattern::new_appliable_multiple(ROOK_MOVES.to_vec())
  }
}
