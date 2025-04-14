use crate::pieces::traits::Movable;
use crate::pieces::types::{SpecialMove, SpecialMoveValidationAction};
use crate::pieces::{Color, Pawn, Position};
use std::collections::HashSet;

#[test]
fn test_white_pawn_initial_moves() {
  let pawn = Pawn::new(Color::White);
  let position = Position::new(1, 4).unwrap(); // e2

  // Closure that simulates an empty board
  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  // Check for valid moves
  for file in 0..8 {
    for rank in 0..8 {
      let target = Position::new(file, rank).unwrap();
      if position != target && pawn.can_reach(position, target, &can_step_into)
      {
        moves.push(target);
      }
    }
  }

  let expected = vec![
    Position::new(2, 4).unwrap(), // e3
    Position::new(3, 4).unwrap(), // e4
  ];

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}

#[test]
fn test_black_pawn_initial_moves() {
  let pawn = Pawn::new(Color::Black);
  let position = Position::new(6, 4).unwrap(); // e7

  // Closure that simulates an empty board
  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  // Check for valid moves
  for file in 0..8 {
    for rank in 0..8 {
      let target = Position::new(file, rank).unwrap();
      if position != target && pawn.can_reach(position, target, &can_step_into)
      {
        moves.push(target);
      }
    }
  }

  let expected = vec![
    Position::new(5, 4).unwrap(), // e6
    Position::new(4, 4).unwrap(), // e5
  ];

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}

#[test]
fn test_white_pawn_after_move() {
  let pawn = Pawn::new(Color::White);
  let position = Position::new(2, 4).unwrap(); // e3

  // Closure that simulates an empty board
  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  // Check for valid moves
  for file in 0..8 {
    for rank in 0..8 {
      let target = Position::new(file, rank).unwrap();
      if position != target && pawn.can_reach(position, target, &can_step_into)
      {
        moves.push(target);
      }
    }
  }

  let expected = vec![
    Position::new(3, 4).unwrap(), // e4
  ];

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}

#[test]
fn test_black_pawn_after_move() {
  let pawn = Pawn::new(Color::Black);
  let position = Position::new(5, 4).unwrap(); // e6

  // Closure that simulates an empty board
  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  // Check for valid moves
  for file in 0..8 {
    for rank in 0..8 {
      let target = Position::new(file, rank).unwrap();
      if position != target && pawn.can_reach(position, target, &can_step_into)
      {
        moves.push(target);
      }
    }
  }

  let expected = vec![
    Position::new(4, 4).unwrap(), // e5
  ];

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}

#[test]
fn test_white_pawn_can_reach_via_en_passant() {
  let pawn = Pawn::new(Color::White);
  let current = Position::new(4, 4).unwrap(); // e5
  let target_left = Position::new(5, 3).unwrap(); // d6
  let target_right = Position::new(5, 5).unwrap(); // f6

  assert_eq!(
    pawn.can_reach_via_special_move(current, target_left),
    Ok(SpecialMove::EnPassant(
      SpecialMoveValidationAction::EnemyPieceExists
    ))
  );

  assert_eq!(
    pawn.can_reach_via_special_move(current, target_right),
    Ok(SpecialMove::EnPassant(
      SpecialMoveValidationAction::EnemyPieceExists
    ))
  );

  let invalid_target = Position::new(5, 4).unwrap(); // e6
  assert_eq!(
    pawn.can_reach_via_special_move(current, invalid_target),
    Err(())
  );
}

#[test]
fn test_black_pawn_can_reach_via_en_passant() {
  let pawn = Pawn::new(Color::Black);
  let current = Position::new(3, 4).unwrap(); // e4
  let target_left = Position::new(2, 3).unwrap(); // d3
  let target_right = Position::new(2, 5).unwrap(); // f3

  assert_eq!(
    pawn.can_reach_via_special_move(current, target_left),
    Ok(SpecialMove::EnPassant(
      SpecialMoveValidationAction::EnemyPieceExists
    ))
  );

  assert_eq!(
    pawn.can_reach_via_special_move(current, target_right),
    Ok(SpecialMove::EnPassant(
      SpecialMoveValidationAction::EnemyPieceExists
    ))
  );

  let invalid_target = Position::new(2, 4).unwrap(); // e3
  assert_eq!(
    pawn.can_reach_via_special_move(current, invalid_target),
    Err(())
  );
}

#[test]
fn test_pawn_upgrade() {
  let pawn = Pawn::new(Color::White);
  let position = Position::new(7, 4).unwrap(); // e7

  assert_eq!(pawn.can_upgrade(position), true);

  let pawn = Pawn::new(Color::Black);
  let position = Position::new(0, 4).unwrap(); // e1

  assert_eq!(pawn.can_upgrade(position), true);

  let invalid_position = Position::new(1, 4).unwrap(); // e2
  assert_eq!(pawn.can_upgrade(invalid_position), false);
}
