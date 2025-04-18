use crate::pieces::traits::Movable;
use crate::pieces::types::color::Color;
use crate::pieces::types::move_direction::{Direction, MovementPattern};
use crate::pieces::types::position::Position;

#[derive(Clone, Copy)]
pub struct King {
  color: Color,
}

const KING_MOVES: [Direction; 8] = [
  Direction::Up,
  Direction::Down,
  Direction::Left,
  Direction::Right,
  Direction::UpLeft,
  Direction::UpRight,
  Direction::DownLeft,
  Direction::DownRight,
];

impl King {
  pub fn new(color: Color) -> Self {
    King { color }
  }
}

impl King {
  pub fn color(&self) -> &Color {
    &self.color
  }
}

impl Movable for King {
  fn movement_pattern(&self, _: Position) -> MovementPattern {
    MovementPattern::new_appliable_once(KING_MOVES.to_vec())
  }
}
