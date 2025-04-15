#[cfg(test)]
mod tests {
  use crate::chessboard::Chessboard;
  use crate::pieces::types::position::Position;

  #[test]
  fn test_standard_board_setup() {
    let board = Chessboard::standard();

    // White back rank
    assert!(board.get_piece(Position::new(0, 0).unwrap()).is_some()); // Rook A1
    assert!(board.get_piece(Position::new(0, 1).unwrap()).is_some()); // Knight B1
    assert!(board.get_piece(Position::new(0, 2).unwrap()).is_some()); // Bishop C1
    assert!(board.get_piece(Position::new(0, 3).unwrap()).is_some()); // Queen D1
    assert!(board.get_piece(Position::new(0, 4).unwrap()).is_some()); // King E1
    assert!(board.get_piece(Position::new(0, 5).unwrap()).is_some()); // Bishop F1
    assert!(board.get_piece(Position::new(0, 6).unwrap()).is_some()); // Knight G1
    assert!(board.get_piece(Position::new(0, 7).unwrap()).is_some()); // Rook H1

    // White pawns
    for y in 0..8 {
      assert!(board.get_piece(Position::new(1, y).unwrap()).is_some());
    }

    // Black back rank
    assert!(board.get_piece(Position::new(7, 0).unwrap()).is_some()); // Rook A8
    assert!(board.get_piece(Position::new(7, 1).unwrap()).is_some()); // Knight B8
    assert!(board.get_piece(Position::new(7, 2).unwrap()).is_some()); // Bishop C8
    assert!(board.get_piece(Position::new(7, 3).unwrap()).is_some()); // Queen D8
    assert!(board.get_piece(Position::new(7, 4).unwrap()).is_some()); // King E8
    assert!(board.get_piece(Position::new(7, 5).unwrap()).is_some()); // Bishop F8
    assert!(board.get_piece(Position::new(7, 6).unwrap()).is_some()); // Knight G8
    assert!(board.get_piece(Position::new(7, 7).unwrap()).is_some()); // Rook H8

    // Black pawns
    for y in 0..8 {
      assert!(board.get_piece(Position::new(6, y).unwrap()).is_some());
    }
  }
}
