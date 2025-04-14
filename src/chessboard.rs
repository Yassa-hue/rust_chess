use crate::chessboard_factory::ChessboardType;
use crate::pieces::{
  Color, Piece, Position, SpecialMove, SpecialMoveValidationAction,
};
use std::collections::HashMap;

pub enum MoveResult {
  None,
  CanUpgradePiece,
}

pub struct Chessboard {
  chessboard: ChessboardType,
  white_dead_pieces: Vec<Box<dyn Piece>>,
  black_dead_pieces: Vec<Box<dyn Piece>>,
}

impl Chessboard {
  pub fn new(chessboard: ChessboardType) -> Self {
    Chessboard {
      chessboard,
      white_dead_pieces: vec![],
      black_dead_pieces: vec![],
    }
  }

  pub fn move_piece(
    &mut self,
    piece_position: Position,
    target_position: Position,
    current_player_color: Color,
  ) -> Result<MoveResult, String> {
    if self.chessboard[piece_position.x()][piece_position.y()].is_none() {
      return Err("No piece at the given position".to_string());
    }

    if !self.can_player_move_piece_at(piece_position, current_player_color) {
      return Err("Not your piece".to_string());
    }

    let moving_piece = self.chessboard[piece_position.x()][piece_position.y()]
      .as_ref()
      .unwrap();

    let can_step_into_postion = |pos: Position| {
      if pos == target_position {
        if let Some(piece) = &self.chessboard[pos.x()][pos.y()] {
          *piece.color() != current_player_color
        } else {
          true
        }
      } else {
        self.chessboard[pos.x()][pos.y()].is_none()
      }
    };

    let res =
      moving_piece.can_reach_via_special_move(piece_position, target_position);

    if !moving_piece.can_reach(
      piece_position,
      target_position,
      &can_step_into_postion,
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
    let piece = self.chessboard[piece_position.x()][piece_position.y()]
      .take()
      .unwrap();

    self.capture_piece(target_position);

    self.chessboard[target_position.x()][target_position.y()] = Some(piece);
    self.chessboard[piece_position.x()][piece_position.y()] = None;

    let piece = self.chessboard[target_position.x()][target_position.y()]
      .as_ref()
      .unwrap();

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
    let dead_pieces = match current_player_color {
      Color::White => &mut self.white_dead_pieces,
      Color::Black => &mut self.black_dead_pieces,
    };

    if piece_index_in_dead_pieces_vector >= dead_pieces.len() {
      return Err("Invalid index for dead pieces vector".to_string());
    }

    let piece_to_upgrade =
      dead_pieces.remove(piece_index_in_dead_pieces_vector);
    self.chessboard[target_position.x()][target_position.y()] =
      Some(piece_to_upgrade);

    Ok(())
  }

  fn can_player_move_piece_at(
    &self,
    position: Position,
    player_color: Color,
  ) -> bool {
    let piece = &self.chessboard[position.x()][position.y()];

    if let Some(piece) = piece {
      if *piece.color() == player_color {
        return true;
      }
    }

    return false;
  }

  fn capture_piece(&mut self, target_position: Position) {
    if let Some(target_piece) =
      self.chessboard[target_position.x()][target_position.y()].take()
    {
      if *target_piece.color() == Color::White {
        self.white_dead_pieces.push(target_piece);
      } else {
        self.black_dead_pieces.push(target_piece);
      }
    }
  }

  fn get_special_move_validation_action(
    &self,
    special_move_validation: SpecialMoveValidationAction,
  ) -> Box<dyn Fn(&mut Chessboard, Position, Position) -> bool> {
    let mut special_move_validation_functions = HashMap::new();
    special_move_validation_functions.insert(
      SpecialMoveValidationAction::EnemyPieceExists,
      |chessboard: &mut Chessboard,
       piece_position: Position,
       target_position: Position| {
        if chessboard.chessboard[target_position.x()][target_position.y()]
          .is_none()
        {
          return false;
        }

        let target_piece = chessboard.chessboard[target_position.x()]
          [target_position.y()]
        .as_ref()
        .unwrap();
        if target_piece.color()
          == chessboard.chessboard[piece_position.x()][piece_position.y()]
            .as_ref()
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

  pub fn chessboard(&self) -> &ChessboardType {
    &self.chessboard
  }

  pub fn black_dead_pieces(&self) -> &Vec<Box<dyn Piece>> {
    &self.black_dead_pieces
  }

  pub fn white_dead_pieces(&self) -> &Vec<Box<dyn Piece>> {
    &self.white_dead_pieces
  }
}
