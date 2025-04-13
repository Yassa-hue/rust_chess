use crate::{
  pieces::types::{Color, MoveOffsets, Position},
  presenters::Presenter,
};

pub trait Piece: Presenter + Movable {
  fn color(&self) -> &Color;
}
pub trait Movable {
  fn can_reach(
    &self,
    current_position: Position,
    target_position: Position,
    can_step_into_postion: &dyn Fn(Position) -> bool,
  ) -> bool {
    let path = self.get_path(current_position, target_position);
    if path.is_none() {
      return false;
    }

    let path = path.unwrap();

    for position in path.iter() {
      if !can_step_into_postion(*position) {
        return false;
      }
    }
    true
  }

  fn get_move_offsets(&self) -> MoveOffsets;

  fn get_path(
    &self,
    current_position: Position,
    target_position: Position,
  ) -> Option<Vec<Position>> {
    let move_offsets = self.get_move_offsets();

    move_offsets.construct_path(current_position, target_position)
  }
}
