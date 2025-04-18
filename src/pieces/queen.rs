use crate::pieces::traits::Movable;
use crate::pieces::types::color::Color;
use crate::pieces::types::move_direction::{Direction, MovementPattern};
use crate::pieces::types::position::Position;

#[derive(Clone, Copy)]
pub struct Queen {
  color: Color,
}

const QUEEN_MOVES: [Direction; 8] = [
  Direction::Up,
  Direction::Down,
  Direction::Left,
  Direction::Right,
  Direction::UpLeft,
  Direction::UpRight,
  Direction::DownLeft,
  Direction::DownRight,
];

impl Queen {
  pub fn new(color: Color) -> Self {
    Queen { color }
  }
}

impl Queen {
  pub fn color(&self) -> &Color {
    &self.color
  }
}

impl Movable for Queen {
  fn movement_pattern(&self, _: Position) -> MovementPattern {
    MovementPattern::new_appliable_multiple(QUEEN_MOVES.to_vec())
  }
}
