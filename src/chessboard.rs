use crate::pieces::types::BOARD_SIZE;
use crate::pieces::types::color::Color;
use crate::pieces::types::position::Position;
use crate::pieces::{Bishop, King, Knight, Pawn, Piece, Queen, Rook};
use std::array::from_fn;

pub type ChessboardType = [[Option<Box<dyn Piece>>; BOARD_SIZE]; BOARD_SIZE];

const FIRST_WHITE_ROW_X_POS: usize = 0;
const WHITE_PAWNS_ROW_X_POS: usize = 1;

const FIRST_BLACK_ROW_X_POS: usize = 7;
const BLACK_PAWNS_ROW_X_POS: usize = 6;

pub struct Chessboard {
  chessboard: ChessboardType,
  white_dead_pieces: Vec<Box<dyn Piece>>,
  black_dead_pieces: Vec<Box<dyn Piece>>,
}

impl Chessboard {
  pub fn new(chessboard: ChessboardType) -> Self {
    Chessboard {
      chessboard,
      white_dead_pieces: Vec::new(),
      black_dead_pieces: Vec::new(),
    }
  }

  pub fn empty() -> Self {
    Self::new(from_fn(|_| from_fn(|_| None)))
  }

  pub fn standard() -> Self {
    let mut board = Chessboard::empty();

    board.initialize_pieces(
      Color::White,
      FIRST_WHITE_ROW_X_POS,
      WHITE_PAWNS_ROW_X_POS,
    );
    board.initialize_pieces(
      Color::Black,
      FIRST_BLACK_ROW_X_POS,
      BLACK_PAWNS_ROW_X_POS,
    );

    board
  }

  fn initialize_pieces(
    &mut self,
    color: Color,
    first_row: usize,
    pawns_row: usize,
  ) {
    let back_row: [Box<dyn Piece>; 8] = [
      Box::new(Rook::new(color)),
      Box::new(Knight::new(color)),
      Box::new(Bishop::new(color)),
      Box::new(Queen::new(color)),
      Box::new(King::new(color)),
      Box::new(Bishop::new(color)),
      Box::new(Knight::new(color)),
      Box::new(Rook::new(color)),
    ];

    for (col, piece) in back_row.into_iter().enumerate() {
      self.set(Position::new(first_row, col).unwrap(), Some(piece));
    }

    for col in 0..BOARD_SIZE {
      self.set(
        Position::new(pawns_row, col).unwrap(),
        Some(Box::new(Pawn::new(color))),
      );
    }
  }

  pub fn get(&self, pos: Position) -> Option<&Box<dyn Piece>> {
    self.chessboard[pos.x()][pos.y()].as_ref()
  }

  pub fn take(&mut self, pos: Position) -> Option<Box<dyn Piece>> {
    self.chessboard[pos.x()][pos.y()].take()
  }

  pub fn set(&mut self, pos: Position, piece: Option<Box<dyn Piece>>) {
    self.chessboard[pos.x()][pos.y()] = piece;
  }

  pub fn board(&self) -> &ChessboardType {
    &self.chessboard
  }

  pub fn white_dead_pieces(&self) -> &Vec<Box<dyn Piece>> {
    &self.white_dead_pieces
  }

  pub fn black_dead_pieces(&self) -> &Vec<Box<dyn Piece>> {
    &self.black_dead_pieces
  }

  pub fn capture_piece(&mut self, target_position: Position) {
    if let Some(target_piece) = self.take(target_position) {
      if *target_piece.color() == Color::White {
        self.white_dead_pieces.push(target_piece);
      } else {
        self.black_dead_pieces.push(target_piece);
      }
    }
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

    self.set(target_position, Some(piece_to_upgrade));

    Ok(())
  }
}
