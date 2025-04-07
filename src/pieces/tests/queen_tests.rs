use std::collections::HashSet;

use crate::pieces::traits::Movable;
use crate::pieces::{Color, Position, Queen};

#[test]
fn test_queen_moves_from_center() {
  let queen = Queen::new(Color::White);
  let pos = Position::new(3, 3);
  let moves = queen.get_valid_moves(pos);

  let mut expected = vec![];

  // Horizontal and vertical
  for i in 0..8 {
    if i != 3 {
      expected.push(Position::new(i, 3)); // same column
      expected.push(Position::new(3, i)); // same row
    }
  }

  // Diagonals
  for i in 1..8 {
    if pos.x() + i < 8 && pos.y() + i < 8 {
      expected.push(Position::new(pos.x() + i, pos.y() + i)); // ↘
    }
    if pos.x() + i < 8 && pos.y() >= i {
      expected.push(Position::new(pos.x() + i, pos.y() - i)); // ↙
    }
    if pos.x() >= i && pos.y() + i < 8 {
      expected.push(Position::new(pos.x() - i, pos.y() + i)); // ↗
    }
    if pos.x() >= i && pos.y() >= i {
      expected.push(Position::new(pos.x() - i, pos.y() - i)); // ↖
    }
  }

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}

#[test]
fn test_queen_moves_from_corner() {
  let queen = Queen::new(Color::Black);
  let pos = Position::new(0, 0);
  let moves = queen.get_valid_moves(pos);

  let mut expected = vec![];

  // Horizontal (right)
  for i in 1..8 {
    expected.push(Position::new(i, 0));
  }

  // Vertical (up)
  for i in 1..8 {
    expected.push(Position::new(0, i));
  }

  // Diagonal ↗ (top-right)
  for i in 1..8 {
    expected.push(Position::new(i, i));
  }

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}
