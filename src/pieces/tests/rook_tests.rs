use std::collections::HashSet;

use crate::pieces::traits::Movable;
use crate::pieces::{Color, Position, Rook};

#[test]
fn test_rook_moves_center() {
  let rook = Rook::new(Color::White);
  let pos = Position::new(4, 4);
  let moves = rook.get_valid_moves(pos);

  let mut expected = vec![];

  // Up
  for i in 1..=4 {
    expected.push(Position::new(4 - i, 4));
  }

  // Down
  for i in 1..=3 {
    expected.push(Position::new(4 + i, 4));
  }

  // Left
  for i in 1..=4 {
    expected.push(Position::new(4, 4 - i));
  }

  // Right
  for i in 1..=3 {
    expected.push(Position::new(4, 4 + i));
  }

  // Order of the moves doesn't matter
  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}

#[test]
fn test_rook_moves_from_corner() {
  let rook = Rook::new(Color::Black);
  let pos = Position::new(0, 0);
  let moves = rook.get_valid_moves(pos);

  let mut expected = vec![];

  for i in 1..8 {
    expected.push(Position::new(i, 0)); // Down
    expected.push(Position::new(0, i)); // Right
  }

  // Order of the moves doesn't matter
  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}
