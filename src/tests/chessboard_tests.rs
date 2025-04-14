#[cfg(test)]
mod tests {
  use crate::chessboard::{Chessboard, MoveResult};
  use crate::chessboard_factory::{ChessboardFactory, ChessboardType};
  use crate::pieces::types::{color::Color, position::Position};
  use crate::pieces::{Pawn, Queen};
  use std::array::from_fn;

  #[test]
  fn test_move_piece_valid_move() {
    let board = ChessboardFactory::standard_board();
    let mut chessboard = Chessboard::new(board);

    // White pawn moves from A2 to A3
    let start_pos = Position::new(1, 0).unwrap(); // A2
    let target_pos = Position::new(2, 0).unwrap(); // A3

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
    let start_pos = Position::new(6, 0).unwrap(); // A7
    let target_pos = Position::new(5, 0).unwrap(); // A6

    let result = chessboard.move_piece(start_pos, target_pos, Color::White);
    assert!(result.is_err()); // Should fail, as it's white's turn
  }

  #[test]
  fn test_capture_piece() {
    let board = ChessboardFactory::standard_board();
    let mut chessboard = Chessboard::new(board);

    // Move a pawn in front of the bishop to free it to move
    chessboard
      .move_piece(
        Position::new(1, 3).unwrap(),
        Position::new(2, 3).unwrap(),
        Color::White,
      )
      .unwrap();

    // Move the black pawn from (6, 7) to (5, 7)
    chessboard
      .move_piece(
        Position::new(6, 7).unwrap(),
        Position::new(5, 7).unwrap(),
        Color::Black,
      )
      .unwrap();

    // Move the white piece from (0, 2) to (5, 7)
    // Assuming the piece at (0, 2) is a bishop (or another piece that can move like this)
    chessboard
      .move_piece(
        Position::new(0, 2).unwrap(),
        Position::new(5, 7).unwrap(),
        Color::White,
      )
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

    let result = chessboard.move_piece(
      Position::new(4, 4).unwrap(),
      Position::new(5, 3).unwrap(),
      Color::White,
    );

    assert!(result.is_ok());
    assert!(chessboard.chessboard()[5][3].is_some()); // White pawn should be on d6
    assert_eq!(chessboard.black_dead_pieces().len(), 1); // Confirm capture
  }

  #[test]
  fn test_pawn_upgrade_triggers_upgrade_result() {
    let mut custom_board: ChessboardType = from_fn(|_| from_fn(|_| None));
    custom_board[6][0] = Some(Box::new(Pawn::new(Color::White)));

    let mut chessboard = Chessboard::new(custom_board);

    let result = chessboard.move_piece(
      Position::new(6, 0).unwrap(),
      Position::new(7, 0).unwrap(),
      Color::White,
    );

    assert!(matches!(result, Ok(MoveResult::CanUpgradePiece)));
  }

  #[test]
  fn test_upgrade_piece_replaces_board_piece() {
    let mut custom_board: ChessboardType = from_fn(|_| from_fn(|_| None));

    // Set up white pawn at 6, 0 (A7) and black pawn at 7, 1 (B8)
    custom_board[6][0] = Some(Box::new(Pawn::new(Color::White)));
    custom_board[7][1] = Some(Box::new(Queen::new(Color::Black)));

    let mut chessboard = Chessboard::new(custom_board);

    // Move white pawn from A7 to B8, capturing the black piece and triggering upgrade
    let result = chessboard.move_piece(
      Position::new(6, 0).unwrap(),
      Position::new(7, 1).unwrap(),
      Color::White,
    );

    assert!(matches!(result, Ok(MoveResult::CanUpgradePiece)));
    assert_eq!(chessboard.black_dead_pieces().len(), 1);

    // Perform upgrade with the captured black piece
    chessboard
      .upgrade_piece(0, Color::Black, Position::new(7, 1).unwrap())
      .expect("Failed to upgrade piece");

    // Confirm the upgrade replaced the pawn on the board
    assert!(chessboard.chessboard()[7][1].is_some());
    assert!(
      !chessboard.chessboard()[7][1]
        .as_ref()
        .unwrap()
        .can_upgrade(Position::new(7, 1).unwrap())
    ); // make sure it's not a pawn anymore
  }
}
