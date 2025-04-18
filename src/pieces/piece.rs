use super::color::Color;
use crate::pieces::types::move_direction::{MovementPattern, SpecialMove};
use crate::pieces::types::position::Position;
use crate::pieces::{Bishop, King, Knight, Movable, Pawn, Queen, Rook};

pub enum Piece {
  Pawn(Pawn),
  Knight(Knight),
  Bishop(Bishop),
  Rook(Rook),
  Queen(Queen),
  King(King),
}

impl Piece {
  pub fn color(&self) -> &Color {
    match self {
      Piece::Pawn(pawn) => pawn.color(),
      Piece::Knight(knight) => knight.color(),
      Piece::Bishop(bishop) => bishop.color(),
      Piece::Rook(rook) => rook.color(),
      Piece::Queen(queen) => queen.color(),
      Piece::King(king) => king.color(),
    }
  }

  pub fn is_of_color(&self, color: Color) -> bool {
    self.color() == &color
  }
}

impl Movable for Piece {
  fn can_reach(
    &self,
    current_position: Position,
    target_position: Position,
    can_step_into_position: &dyn Fn(Position) -> bool,
  ) -> bool {
    match self {
      Piece::Pawn(p) => {
        p.can_reach(current_position, target_position, can_step_into_position)
      }
      Piece::Knight(p) => {
        p.can_reach(current_position, target_position, can_step_into_position)
      }
      Piece::Bishop(p) => {
        p.can_reach(current_position, target_position, can_step_into_position)
      }
      Piece::Rook(p) => {
        p.can_reach(current_position, target_position, can_step_into_position)
      }
      Piece::Queen(p) => {
        p.can_reach(current_position, target_position, can_step_into_position)
      }
      Piece::King(p) => {
        p.can_reach(current_position, target_position, can_step_into_position)
      }
    }
  }

  fn can_reach_via_special_move(
    &self,
    current_position: Position,
    target_position: Position,
  ) -> Result<SpecialMove, ()> {
    match self {
      Piece::Pawn(p) => {
        p.can_reach_via_special_move(current_position, target_position)
      }
      Piece::King(p) => {
        p.can_reach_via_special_move(current_position, target_position)
      }
      _ => Err(()),
    }
  }

  fn movement_pattern(&self, start_position: Position) -> MovementPattern {
    match self {
      Piece::Pawn(p) => p.movement_pattern(start_position),
      Piece::Knight(p) => p.movement_pattern(start_position),
      Piece::Bishop(p) => p.movement_pattern(start_position),
      Piece::Rook(p) => p.movement_pattern(start_position),
      Piece::Queen(p) => p.movement_pattern(start_position),
      Piece::King(p) => p.movement_pattern(start_position),
    }
  }

  fn get_path(
    &self,
    current_position: Position,
    target_position: Position,
  ) -> Option<Vec<Position>> {
    match self {
      Piece::Pawn(p) => p.get_path(current_position, target_position),
      Piece::Knight(p) => p.get_path(current_position, target_position),
      Piece::Bishop(p) => p.get_path(current_position, target_position),
      Piece::Rook(p) => p.get_path(current_position, target_position),
      Piece::Queen(p) => p.get_path(current_position, target_position),
      Piece::King(p) => p.get_path(current_position, target_position),
    }
  }
}
