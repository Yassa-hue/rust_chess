use std::collections::HashSet;

use crate::pieces::traits::Movable;
use crate::pieces::{Color, King, Position};

#[test]
fn test_king_moves_center() {
  let king = King::new(Color::White);
  let pos = Position::new(4, 4).unwrap();

  // Closure that simulates an empty board
  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  for file in 0..8 {
    for rank in 0..8 {
      let target = Position::new(file, rank).unwrap();
      if pos != target && king.can_reach(pos, target, &can_step_into) {
        moves.push(target);
      }
    }
  }

  let expected = vec![
    Position::new(5, 4).unwrap(),
    Position::new(3, 4).unwrap(),
    Position::new(4, 5).unwrap(),
    Position::new(4, 3).unwrap(),
    Position::new(5, 5).unwrap(),
    Position::new(5, 3).unwrap(),
    Position::new(3, 5).unwrap(),
    Position::new(3, 3).unwrap(),
  ];

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}

#[test]
fn test_king_moves_corner() {
  let king = King::new(Color::Black);
  let pos = Position::new(0, 0).unwrap();

  // Closure that simulates an empty board
  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  for file in 0..8 {
    for rank in 0..8 {
      let target = Position::new(file, rank).unwrap();
      if pos != target && king.can_reach(pos, target, &can_step_into) {
        moves.push(target);
      }
    }
  }

  let expected = vec![
    Position::new(1, 0).unwrap(),
    Position::new(0, 1).unwrap(),
    Position::new(1, 1).unwrap(),
  ];

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}
