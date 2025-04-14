use crate::pieces::types::color::Color;
use crate::pieces::types::move_direction::{MoveOffsets, SpecialMove};
use crate::pieces::types::position::Position;
use crate::presenters::Presenter;

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

  fn can_reach_via_special_move(
    &self,
    _: Position,
    _: Position,
  ) -> Result<SpecialMove, ()> {
    Err(())
  }

  fn get_move_offsets(&self, start_position: Position) -> MoveOffsets;

  fn get_path(
    &self,
    current_position: Position,
    target_position: Position,
  ) -> Option<Vec<Position>> {
    let move_offsets = self.get_move_offsets(current_position);

    move_offsets.construct_path(current_position, target_position)
  }

  fn can_upgrade(&self, _: Position) -> bool {
    false
  }
}
