use std::collections::HashSet;

use crate::pieces::traits::Movable;
use crate::pieces::{Color, Position, Rook};

#[test]
fn test_rook_moves_center() {
  let rook = Rook::new(Color::White);
  let pos = Position::new(4, 4);

  // Closure that simulates an empty board
  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  // Up
  for i in 1..=4 {
    if rook.can_reach(pos, Position::new(4 - i, 4), &can_step_into) {
      moves.push(Position::new(4 - i, 4));
    }
  }

  // Down
  for i in 1..=3 {
    if rook.can_reach(pos, Position::new(4 + i, 4), &can_step_into) {
      moves.push(Position::new(4 + i, 4));
    }
  }

  // Left
  for i in 1..=4 {
    if rook.can_reach(pos, Position::new(4, 4 - i), &can_step_into) {
      moves.push(Position::new(4, 4 - i));
    }
  }

  // Right
  for i in 1..=3 {
    if rook.can_reach(pos, Position::new(4, 4 + i), &can_step_into) {
      moves.push(Position::new(4, 4 + i));
    }
  }

  let expected = moves.clone(); // expected positions already generated from the code

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}

#[test]
fn test_rook_moves_from_corner() {
  let rook = Rook::new(Color::Black);
  let pos = Position::new(0, 0);

  // Closure that simulates an empty board
  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  // Down
  for i in 1..8 {
    if rook.can_reach(pos, Position::new(i, 0), &can_step_into) {
      moves.push(Position::new(i, 0));
    }
  }

  // Right
  for i in 1..8 {
    if rook.can_reach(pos, Position::new(0, i), &can_step_into) {
      moves.push(Position::new(0, i));
    }
  }

  let expected = moves.clone(); // expected positions already generated from the code

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}
