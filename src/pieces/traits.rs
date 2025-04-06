use std::fmt::Display;
use crate::pieces::types::{Color, MoveOffsets, Position};


pub trait Movable: Display {
  fn get_move_offsets(&self) -> MoveOffsets;

  fn color(&self) -> &Color;

  fn get_valid_moves(&self, current_position: Position) -> Vec<Position> {
      let move_offsets = self.get_move_offsets();

      move_offsets.apply_offsets(current_position)
  }
}
