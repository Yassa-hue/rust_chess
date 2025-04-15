use crate::chessboard::Chessboard;
use crate::pieces::types::color::Color;
use crate::pieces::types::move_direction::{
  SpecialMove, SpecialMoveValidationAction,
};
use crate::pieces::types::position::Position;
use std::collections::HashMap;

pub enum MoveResult {
  None,
  CanUpgradePiece,
}

pub struct BoardManager {
  chessboard: Chessboard,
}

impl BoardManager {
  pub fn new(chessboard: Chessboard) -> Self {
    BoardManager { chessboard }
  }

  pub fn move_piece(
    &mut self,
    piece_position: Position,
    target_position: Position,
    current_player_color: Color,
  ) -> Result<MoveResult, String> {
    if self.chessboard.get(piece_position).is_none() {
      return Err("No piece at the given position".to_string());
    }

    if !self.can_player_move_piece_at(piece_position, current_player_color) {
      return Err("Not your piece".to_string());
    }

    let moving_piece = self.chessboard.get(piece_position).unwrap();

    let can_step_into_position = |pos: Position| {
      if pos == target_position {
        if let Some(piece) = &self.chessboard.get(pos) {
          *piece.color() != current_player_color
        } else {
          true
        }
      } else {
        self.chessboard.get(pos).is_none()
      }
    };

    let res =
      moving_piece.can_reach_via_special_move(piece_position, target_position);

    if !moving_piece.can_reach(
      piece_position,
      target_position,
      &can_step_into_position,
    ) && res.is_err()
    {
      return Err("Invalid move".to_string());
    }

    let special_move_validation = match res {
      Ok(special_move) => match special_move {
        SpecialMove::EnPassant(action) => Some(action),
      },
      Err(_) => None,
    };

    match special_move_validation {
      Some(validation_action) => {
        let f = self.get_special_move_validation_action(validation_action);
        if !f(self, piece_position, target_position) {
          return Err("Invalid special move".to_string());
        }
      }
      None => {}
    }

    // apply the move safely
    let piece = self.chessboard.take(piece_position).unwrap();

    self.chessboard.capture_piece(target_position);

    self.chessboard.set(target_position, Some(piece));
    self.chessboard.set(piece_position, None);

    let piece = self.chessboard.get(target_position).unwrap();

    if piece.can_upgrade(target_position) {
      return Ok(MoveResult::CanUpgradePiece);
    }

    return Ok(MoveResult::None);
  }

  pub fn upgrade_piece(
    &mut self,
    piece_index_in_dead_pieces_vector: usize,
    current_player_color: Color,
    target_position: Position,
  ) -> Result<(), String> {
    self.chessboard.upgrade_piece(
      piece_index_in_dead_pieces_vector,
      current_player_color,
      target_position,
    )
  }

  fn can_player_move_piece_at(
    &self,
    position: Position,
    player_color: Color,
  ) -> bool {
    if let Some(piece) = self.chessboard.get(position) {
      return *piece.color() == player_color;
    }
    false
  }

  fn get_special_move_validation_action(
    &self,
    special_move_validation: SpecialMoveValidationAction,
  ) -> Box<dyn Fn(&mut BoardManager, Position, Position) -> bool> {
    let mut special_move_validation_functions = HashMap::new();
    special_move_validation_functions.insert(
      SpecialMoveValidationAction::EnemyPieceExists,
      |board_manager: &mut BoardManager,
       piece_position: Position,
       target_position: Position| {
        if board_manager.chessboard.get(target_position).is_none() {
          return false;
        }

        let target_piece =
          board_manager.chessboard.get(target_position).unwrap();
        if target_piece.color()
          == board_manager
            .chessboard
            .get(piece_position)
            .unwrap()
            .color()
        {
          return false;
        }
        true
      },
    );

    Box::new(
      special_move_validation_functions
        .remove(&special_move_validation)
        .unwrap(),
    )
  }

  pub fn chessboard(&self) -> &Chessboard {
    &self.chessboard
  }
}
