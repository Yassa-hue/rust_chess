use crate::pieces::traits::Movable;
use crate::pieces::{Color, Pawn, Position};
use std::collections::HashSet;

#[test]
fn test_white_pawn_initial_moves() {
  let pawn = Pawn::new(Color::White);
  let position = Position::new(1, 4); // e2

  // Closure that simulates an empty board
  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  // Check for valid moves
  for file in 0..8 {
    for rank in 0..8 {
      let target = Position::new(file, rank);
      if position != target && pawn.can_reach(position, target, &can_step_into) {
        moves.push(target);
      }
    }
  }

  let expected = vec![
    Position::new(2, 4), // e3
    Position::new(3, 4), // e4
  ];

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}

#[test]
fn test_black_pawn_initial_moves() {
  let pawn = Pawn::new(Color::Black);
  let position = Position::new(6, 4); // e7

  // Closure that simulates an empty board
  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  // Check for valid moves
  for file in 0..8 {
    for rank in 0..8 {
      let target = Position::new(file, rank);
      if position != target && pawn.can_reach(position, target, &can_step_into) {
        moves.push(target);
      }
    }
  }

  let expected = vec![
    Position::new(5, 4), // e6
    Position::new(4, 4), // e5
  ];

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}

#[test]
fn test_white_pawn_after_move() {
  let pawn = Pawn::new(Color::White);
  let position = Position::new(2, 4); // e3

  // Closure that simulates an empty board
  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  // Check for valid moves
  for file in 0..8 {
    for rank in 0..8 {
      let target = Position::new(file, rank);
      if position != target && pawn.can_reach(position, target, &can_step_into) {
        moves.push(target);
      }
    }
  }

  let expected = vec![
    Position::new(3, 4), // e4
  ];

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}

#[test]
fn test_black_pawn_after_move() {
  let pawn = Pawn::new(Color::Black);
  let position = Position::new(5, 4); // e6

  // Closure that simulates an empty board
  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  // Check for valid moves
  for file in 0..8 {
    for rank in 0..8 {
      let target = Position::new(file, rank);
      if position != target && pawn.can_reach(position, target, &can_step_into) {
        moves.push(target);
      }
    }
  }

  let expected = vec![
    Position::new(4, 4), // e5
  ];

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}
