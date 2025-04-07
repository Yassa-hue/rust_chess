use std::collections::HashSet;

use crate::pieces::traits::Movable;
use crate::pieces::{Color, King, Position};

#[test]
fn test_king_moves_center() {
  let king = King::new(Color::White);
  let pos = Position::new(4, 4);
  let moves = king.get_valid_moves(pos);

  let expected = vec![
    Position::new(5, 4),
    Position::new(3, 4),
    Position::new(4, 5),
    Position::new(4, 3),
    Position::new(5, 5),
    Position::new(5, 3),
    Position::new(3, 5),
    Position::new(3, 3),
  ];

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}

#[test]
fn test_king_moves_corner() {
  let king = King::new(Color::Black);
  let pos = Position::new(0, 0);
  let moves = king.get_valid_moves(pos);

  let expected = vec![
    Position::new(1, 0),
    Position::new(0, 1),
    Position::new(1, 1),
  ];

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}
