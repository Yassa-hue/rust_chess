#[cfg(test)]
mod tests {
  use crate::chessboard::Chessboard;
  use crate::pieces::{Color, Position};

  #[test]
  fn test_initialize_chessboard() {
    let chessboard = Chessboard::new();

    // Check if white pieces are in the correct positions
    assert!(chessboard.chessboard()[0][0].is_some()); // White Rook at A1
    assert!(chessboard.chessboard()[0][1].is_some()); // White Knight at B1
    assert!(chessboard.chessboard()[0][2].is_some()); // White Bishop at C1
    assert!(chessboard.chessboard()[0][3].is_some()); // White Queen at D1
    assert!(chessboard.chessboard()[0][4].is_some()); // White King at E1

    // Check if black pieces are in the correct positions
    assert!(chessboard.chessboard()[7][0].is_some()); // Black Rook at A8
    assert!(chessboard.chessboard()[7][1].is_some()); // Black Knight at B8
    assert!(chessboard.chessboard()[7][2].is_some()); // Black Bishop at C8
    assert!(chessboard.chessboard()[7][3].is_some()); // Black Queen at D8
    assert!(chessboard.chessboard()[7][4].is_some()); // Black King at E8
  }

  #[test]
  fn test_move_piece_valid_move() {
    let mut chessboard = Chessboard::new();

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
    let mut chessboard = Chessboard::new();

    // Try to move a black pawn from A7 to A6 when it's white's turn
    let start_pos = Position::new(6, 0); // A7
    let target_pos = Position::new(5, 0); // A6

    let result = chessboard.move_piece(start_pos, target_pos, Color::White);
    assert!(result.is_err()); // Should fail, as it's white's turn
  }

  #[test]
  fn test_capture_piece() {
    let mut chessboard = Chessboard::new();

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
  fn test_is_clear_of_pieces_of_color() {
    let chessboard = Chessboard::new();

    // The position A2 (1, 0) should be occupied by a white pawn initially
    assert!(!chessboard.is_clear_of_pieces_of_color(Color::White, Position::new(1, 0)));

    // The position D4 (3, 3) should be clear of pieces initially
    assert!(chessboard.is_clear_of_pieces_of_color(Color::White, Position::new(3, 3)));
  }

  #[test]
  fn test_is_valid_move_for_piece() {
    let chessboard = Chessboard::new();

    // White pawn at A2 (1, 0) should have valid moves to A3 (2, 0) and A4 (3, 0)
    let start_pos = Position::new(1, 0); // A2
    let valid_move = Position::new(2, 0); // A3
    assert!(chessboard.is_valid_path(start_pos, valid_move, Color::White));

    // Invalid move for a white pawn (A2 to A5, not a valid first move)
    let invalid_move = Position::new(4, 0); // A5
    assert!(!chessboard.is_valid_path(start_pos, invalid_move, Color::White));
  }
}
