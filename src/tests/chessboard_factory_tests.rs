#[cfg(test)]
mod tests {
  use crate::chessboard_factory::ChessboardFactory;

  #[test]
  fn test_standard_board_setup() {
    let board = ChessboardFactory::standard_board();

    // White back rank
    assert!(board[0][0].is_some()); // Rook A1
    assert!(board[0][1].is_some()); // Knight B1
    assert!(board[0][2].is_some()); // Bishop C1
    assert!(board[0][3].is_some()); // Queen D1
    assert!(board[0][4].is_some()); // King E1
    assert!(board[0][5].is_some()); // Bishop
    assert!(board[0][6].is_some()); // Knight
    assert!(board[0][7].is_some()); // Rook

    // White pawns
    for y in 0..8 {
      assert!(board[1][y].is_some());
    }

    // Black back rank
    assert!(board[7][0].is_some()); // Rook A8
    assert!(board[7][1].is_some()); // Knight B8
    assert!(board[7][2].is_some()); // Bishop C8
    assert!(board[7][3].is_some()); // Queen D8
    assert!(board[7][4].is_some()); // King E8
    assert!(board[7][5].is_some()); // Bishop
    assert!(board[7][6].is_some()); // Knight
    assert!(board[7][7].is_some()); // Rook

    // Black pawns
    for y in 0..8 {
      assert!(board[6][y].is_some());
    }
  }
}
