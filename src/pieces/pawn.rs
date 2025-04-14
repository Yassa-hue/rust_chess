use crate::pieces::traits::{Movable, Piece};
use crate::pieces::types::{
  Color, MoveOffsets, Position, SpecialMove, SpecialMoveValidationAction,
};

const PAWN_X_START_POSITIONS: [usize; 2] = [
  1, // White
  6, // Black
];

const WHITE_PAWN_UPGRADE_X_POSITION: usize = 7;
const BLACK_PAWN_UPGRADE_X_POSITION: usize = 0;

#[derive(Clone, Copy)]
pub struct Pawn {
  color: Color,
}

impl Pawn {
  pub fn new(color: Color) -> Self {
    Pawn { color }
  }

  fn get_en_passant_move_offsets(&self) -> MoveOffsets {
    let offsets = match self.color {
      Color::White => vec![(1, -1), (1, 1)],
      Color::Black => vec![(-1, -1), (-1, 1)],
    };

    MoveOffsets::new_appliable_once(offsets)
  }
}

impl Piece for Pawn {
  fn color(&self) -> &Color {
    &self.color
  }
}

impl Movable for Pawn {
  fn get_move_offsets(&self, current_position: Position) -> MoveOffsets {
    let offsets = match self.color {
      Color::White => vec![(1, 0)],
      Color::Black => vec![(-1, 0)],
    };

    // Pawns can move two squares forward from their starting position
    if PAWN_X_START_POSITIONS
      .iter()
      .any(|x| *x == current_position.x())
    {
      MoveOffsets::new_appliable_twice(offsets)
    } else {
      // Pawns can only move one square forward otherwise
      MoveOffsets::new_appliable_once(offsets)
    }
  }

  fn can_reach_via_special_move(
    &self,
    current_position: Position,
    target_position: Position,
  ) -> Result<SpecialMove, ()> {
    let move_offsets = self.get_en_passant_move_offsets();

    if let MoveOffsets::AppliableOnce(offsets) = move_offsets {
      for offset in offsets {
        let target_x = current_position.x() as i32 + offset.0;
        let target_y = current_position.y() as i32 + offset.1;

        if target_x == target_position.x() as i32 && target_y == target_position.y() as i32 {
          return Ok(SpecialMove::EnPassant(
            SpecialMoveValidationAction::EnemyPieceExists,
          ));
        }
      }
    }

    Err(())
  }

  fn can_upgrade(&self, current_position: Position) -> bool {
    match self.color {
      Color::White => current_position.x() == WHITE_PAWN_UPGRADE_X_POSITION,
      Color::Black => current_position.x() == BLACK_PAWN_UPGRADE_X_POSITION,
    }
  }
}
