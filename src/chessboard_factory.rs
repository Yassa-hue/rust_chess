use crate::pieces::{BOARD_SIZE, Bishop, Color, King, Knight, Pawn, Piece, Queen, Rook};
use std::array::from_fn;

const FIRST_WHITE_ROW_X_POS: usize = 0;
const WHITE_POWNS_ROW_X_POS: usize = 1;

const FIRST_BLACK_ROW_X_POS: usize = 7;
const BLACK_POWNS_ROW_X_POS: usize = 6;

type ChessboardType = [[Option<Box<dyn Piece>>; BOARD_SIZE]; BOARD_SIZE];
pub struct ChessboardFactory;

impl ChessboardFactory {
  pub fn standard_board() -> ChessboardType {
    let mut board: ChessboardType = from_fn(|_| from_fn(|_| None));

    Self::initialize_pieces(
      &mut board,
      Color::White,
      FIRST_WHITE_ROW_X_POS,
      WHITE_POWNS_ROW_X_POS,
    );
    Self::initialize_pieces(
      &mut board,
      Color::Black,
      FIRST_BLACK_ROW_X_POS,
      BLACK_POWNS_ROW_X_POS,
    );

    board
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
}
