use crate::pieces::types::move_direction::{MovementPattern, SpecialMove};
use crate::pieces::types::position::Position;

pub trait Movable {
  fn can_reach(
    &self,
    current_position: Position,
    target_position: Position,
    can_step_into_position: &dyn Fn(Position) -> bool,
  ) -> bool {
    let path = self.get_path(current_position, target_position);
    if path.is_none() {
      return false;
    }

    let path = path.unwrap();

    for position in path.iter() {
      if !can_step_into_position(*position) {
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

  fn movement_pattern(&self, start_position: Position) -> MovementPattern;

  fn get_path(
    &self,
    current_position: Position,
    target_position: Position,
  ) -> Option<Vec<Position>> {
    let movement_pattern = self.movement_pattern(current_position);

    movement_pattern.construct_path(current_position, target_position)
  }
}
