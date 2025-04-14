use std::collections::HashSet;

use crate::pieces::knight::Knight;
use crate::pieces::traits::Movable;
use crate::pieces::types::{color::Color, position::Position};

#[test]
fn test_knight_moves_center() {
  let knight = Knight::new(Color::White);
  let pos = Position::new(4, 4).unwrap();

  // Closure that simulates an empty board
  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  for file in 0..8 {
    for rank in 0..8 {
      let target = Position::new(file, rank).unwrap();
      if pos != target && knight.can_reach(pos, target, &can_step_into) {
        moves.push(target);
      }
    }
  }

  let expected = vec![
    Position::new(6, 5).unwrap(),
    Position::new(6, 3).unwrap(),
    Position::new(2, 5).unwrap(),
    Position::new(2, 3).unwrap(),
    Position::new(5, 6).unwrap(),
    Position::new(5, 2).unwrap(),
    Position::new(3, 6).unwrap(),
    Position::new(3, 2).unwrap(),
  ];

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}

#[test]
fn test_knight_moves_near_edge() {
  let knight = Knight::new(Color::Black);
  let pos = Position::new(0, 0).unwrap();

  // Closure that simulates an empty board
  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  for file in 0..8 {
    for rank in 0..8 {
      let target = Position::new(file, rank).unwrap();
      if pos != target && knight.can_reach(pos, target, &can_step_into) {
        moves.push(target);
      }
    }
  }

  let expected =
    vec![Position::new(2, 1).unwrap(), Position::new(1, 2).unwrap()];

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}
