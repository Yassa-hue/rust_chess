use crate::pieces::traits::{Movable, Piece};
use crate::pieces::types::color::Color;
use crate::pieces::types::move_direction::{Direction, MovementPattern};
use crate::pieces::types::position::Position;

#[derive(Clone, Copy)]
pub struct Bishop {
  color: Color,
}

const BISHOP_MOVES: [Direction; 4] = [
  Direction::UpRight,
  Direction::UpLeft,
  Direction::DownRight,
  Direction::DownLeft,
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
  fn movement_pattern(&self, _: Position) -> MovementPattern {
    MovementPattern::new_appliable_multiple(BISHOP_MOVES.to_vec())
  }
}
