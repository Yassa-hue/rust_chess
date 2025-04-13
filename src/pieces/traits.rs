use crate::{
  pieces::types::{Color, MoveOffsets, Position},
  presenters::Presenter,
};

pub trait Piece: Presenter + Movable {
  fn color(&self) -> &Color;
}
pub trait Movable {
  fn get_move_offsets(&self) -> MoveOffsets;

  fn get_valid_moves(&self, current_position: Position) -> Vec<Position> {
    let move_offsets = self.get_move_offsets();

    move_offsets.apply_offsets(current_position)
  }

  fn is_movement_include_multible_steps(&self) -> bool;
}
