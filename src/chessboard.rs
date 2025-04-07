use crate::pieces::{BOARD_SIZE, Bishop, Color, King, Knight, Pawn, Piece, Position, Queen, Rook};
use std::array::from_fn;

const FIRST_WHITE_ROW_X_POS: usize = 0;
const WHITE_POWNS_ROW_X_POS: usize = 1;

const FIRST_BLACK_ROW_X_POS: usize = 7;
const BLACK_POWNS_ROW_X_POS: usize = 6;

type ChessboardType = [[Option<Box<dyn Piece>>; BOARD_SIZE]; BOARD_SIZE];

pub struct Chessboard {
  chessboard: ChessboardType,
  white_dead_pieces: Vec<Box<dyn Piece>>,
  black_dead_pieces: Vec<Box<dyn Piece>>,
}

impl Chessboard {
  pub fn new() -> Self {
    let mut chessboard: ChessboardType = from_fn(|_| from_fn(|_| None));

    Self::initialize_pieces(
      &mut chessboard,
      Color::White,
      FIRST_WHITE_ROW_X_POS,
      WHITE_POWNS_ROW_X_POS,
    );
    Self::initialize_pieces(
      &mut chessboard,
      Color::Black,
      FIRST_BLACK_ROW_X_POS,
      BLACK_POWNS_ROW_X_POS,
    );

    Chessboard {
      chessboard,
      white_dead_pieces: vec![],
      black_dead_pieces: vec![],
    }
  }

  fn initialize_pieces(
    chessboard: &mut ChessboardType,
    color: Color,
    first_row_pos: usize,
    pawns_row_pos: usize,
  ) -> () {
    let first_row: [Option<Box<dyn Piece>>; BOARD_SIZE] = [
      Some(Box::new(Rook::new(color))),
      Some(Box::new(Knight::new(color))),
      Some(Box::new(Bishop::new(color))),
      Some(Box::new(Queen::new(color))),
      Some(Box::new(King::new(color))),
      Some(Box::new(Bishop::new(color))),
      Some(Box::new(Knight::new(color))),
      Some(Box::new(Rook::new(color))),
    ];

    for (col, piece) in first_row.into_iter().enumerate() {
      chessboard[first_row_pos][col] = piece;
    }

    for col in 0..BOARD_SIZE {
      chessboard[pawns_row_pos][col] = Some(Box::new(Pawn::new(color)));
    }
  }

  pub fn is_clear_of_pieces_of_color(&self, color: Color, position: Position) -> bool {
    let pos = &self.chessboard[position.x()][position.y()];

    match pos {
      Some(piece) if *piece.color() == color => false,
      _ => true,
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

    if !self.is_valid_path(piece_position, target_position, current_player_color) {
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

  pub fn is_valid_path(
    &self,
    start_pos: Position,
    final_distination: Position,
    player_color: Color,
  ) -> bool {
    let moving_piece = self.chessboard[start_pos.x()][start_pos.y()]
      .as_ref()
      .unwrap();

    if !moving_piece
      .get_valid_moves(start_pos)
      .iter()
      .filter(|position| self.is_clear_of_pieces_of_color(player_color, **position))
      .any(|position| *position == final_distination)
    {
      return false;
    }

    return true;
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

  // for testing purpose
  #[allow(dead_code)]
  pub fn black_dead_pieces(&self) -> &Vec<Box<dyn Piece>> {
    &self.black_dead_pieces
  }

  // for testing purpose
  #[allow(dead_code)]
  pub fn white_dead_pieces(&self) -> &Vec<Box<dyn Piece>> {
    &self.white_dead_pieces
  }
}
