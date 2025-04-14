#[cfg(test)]
mod tests {
  use crate::chessboard::Chessboard;
  use crate::chessboard_factory::{ChessboardFactory, ChessboardType};
  use crate::pieces::Pawn;
  use crate::pieces::{Color, Position};
  use std::array::from_fn;

  #[test]
  fn test_move_piece_valid_move() {
    let board = ChessboardFactory::standard_board();
    let mut chessboard = Chessboard::new(board);

    // White pawn moves from A2 to A3
    let start_pos = Position::new(1, 0); // A2
    let target_pos = Position::new(2, 0); // A3

    assert!(
      chessboard
        .move_piece(start_pos, target_pos, Color::White)
        .is_ok()
    );
    assert!(chessboard.chessboard()[2][0].is_some()); // The pawn should now be at A3
    assert!(chessboard.chessboard()[1][0].is_none()); // The original position should be empty
  }

  #[test]
  fn test_move_piece_invalid_move() {
    let board = ChessboardFactory::standard_board();
    let mut chessboard = Chessboard::new(board);

    // Try to move a black pawn from A7 to A6 when it's white's turn
    let start_pos = Position::new(6, 0); // A7
    let target_pos = Position::new(5, 0); // A6

    let result = chessboard.move_piece(start_pos, target_pos, Color::White);
    assert!(result.is_err()); // Should fail, as it's white's turn
  }

  #[test]
  fn test_capture_piece() {
    let board = ChessboardFactory::standard_board();
    let mut chessboard = Chessboard::new(board);

    // Move a pawn in front of the bishop to free it to move
    chessboard
      .move_piece(Position::new(1, 3), Position::new(2, 3), Color::White)
      .unwrap();

    // Move the black pawn from (6, 7) to (5, 7)
    chessboard
      .move_piece(Position::new(6, 7), Position::new(5, 7), Color::Black)
      .unwrap();

    // Move the white piece from (0, 2) to (5, 7)
    // Assuming the piece at (0, 2) is a bishop (or another piece that can move like this)
    chessboard
      .move_piece(Position::new(0, 2), Position::new(5, 7), Color::White)
      .unwrap();

    // The bishop captures the pawn, so it should now be in the dead pieces list
    assert_eq!(chessboard.black_dead_pieces().len(), 1);
  }

  #[test]
  fn test_en_passant_capture() {
    // Custom board setup
    let mut custom_board: ChessboardType = from_fn(|_| from_fn(|_| None));

    custom_board[4][4] = Some(Box::new(Pawn::new(Color::White)));
    custom_board[5][3] = Some(Box::new(Pawn::new(Color::Black)));

    let mut chessboard = Chessboard::new(custom_board);

    let result = chessboard.move_piece(Position::new(4, 4), Position::new(5, 3), Color::White);

    assert!(result.is_ok());
    assert!(chessboard.chessboard()[5][3].is_some()); // White pawn should be on d6
    assert_eq!(chessboard.black_dead_pieces().len(), 1); // Confirm capture
  }
}
