use crate::pieces::{Color, Piece, Position};

use crate::chessboard_factory::ChessboardType;

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
  ) -> Result<(), String> {
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

    if !moving_piece.can_reach(piece_position, target_position, &can_step_into_postion) {
      return Err("Invalid move".to_string());
    }

    // apply the move safly
    let piece = self.chessboard[piece_position.x()][piece_position.y()]
      .take()
      .unwrap();

    self.capture_piece(target_position);

    self.chessboard[target_position.x()][target_position.y()] = Some(piece);
    self.chessboard[piece_position.x()][piece_position.y()] = None;

    return Ok(());
  }

  fn can_player_move_piece_at(&self, position: Position, player_color: Color) -> bool {
    let piece = &self.chessboard[position.x()][position.y()];

    if let Some(piece) = piece {
      if *piece.color() == player_color {
        return true;
      }
    }

    return false;
  }

  fn capture_piece(&mut self, target_position: Position) {
    if let Some(target_piece) = self.chessboard[target_position.x()][target_position.y()].take() {
      if *target_piece.color() == Color::White {
        self.white_dead_pieces.push(target_piece);
      } else {
        self.black_dead_pieces.push(target_piece);
      }
    }
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
