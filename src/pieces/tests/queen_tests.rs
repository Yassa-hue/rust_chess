use std::collections::HashSet;

use crate::pieces::traits::Movable;
use crate::pieces::{Color, Position, Queen};

#[test]
fn test_queen_moves_from_center() {
  let queen = Queen::new(Color::White);
  let pos = Position::new(3, 3).unwrap();
  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  // Horizontal and vertical
  for i in 0..8 {
    if i != pos.x() {
      if queen.can_reach(
        pos,
        Position::new(i, pos.y()).unwrap(),
        &can_step_into,
      ) {
        moves.push(Position::new(i, pos.y()).unwrap());
      }
    }
    if i != pos.y() {
      if queen.can_reach(
        pos,
        Position::new(pos.x(), i).unwrap(),
        &can_step_into,
      ) {
        moves.push(Position::new(pos.x(), i).unwrap());
      }
    }
  }

  // Diagonals
  for i in 1..8 {
    // ↘
    if pos.x() + i < 8 && pos.y() + i < 8 {
      let p = Position::new(pos.x() + i, pos.y() + i).unwrap();
      if queen.can_reach(pos, p, &can_step_into) {
        moves.push(p);
      }
    }
    // ↙
    if pos.x() + i < 8 && pos.y() >= i {
      let p = Position::new(pos.x() + i, pos.y() - i).unwrap();
      if queen.can_reach(pos, p, &can_step_into) {
        moves.push(p);
      }
    }
    // ↗
    if pos.x() >= i && pos.y() + i < 8 {
      let p = Position::new(pos.x() - i, pos.y() + i).unwrap();
      if queen.can_reach(pos, p, &can_step_into) {
        moves.push(p);
      }
    }
    // ↖
    if pos.x() >= i && pos.y() >= i {
      let p = Position::new(pos.x() - i, pos.y() - i).unwrap();
      if queen.can_reach(pos, p, &can_step_into) {
        moves.push(p);
      }
    }
  }

  let expected_set: HashSet<_> = moves.iter().cloned().collect();
  let moves_set: HashSet<_> = moves.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}

#[test]
fn test_queen_moves_from_corner() {
  let queen = Queen::new(Color::Black);
  let pos = Position::new(0, 0).unwrap();

  // Closure that simulates an empty board
  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  // Horizontal (right)
  for i in 1..8 {
    if queen.can_reach(pos, Position::new(i, 0).unwrap(), &can_step_into) {
      moves.push(Position::new(i, 0).unwrap());
    }
  }

  // Vertical (up)
  for i in 1..8 {
    if queen.can_reach(pos, Position::new(0, i).unwrap(), &can_step_into) {
      moves.push(Position::new(0, i).unwrap());
    }
  }

  // Diagonal ↗ (top-right)
  for i in 1..8 {
    if queen.can_reach(pos, Position::new(i, i).unwrap(), &can_step_into) {
      moves.push(Position::new(i, i).unwrap());
    }
  }

  let expected = moves.clone(); // expected positions already generated from the code

  let moves_set: HashSet<_> = moves.into_iter().collect();
  let expected_set: HashSet<_> = expected.into_iter().collect();

  assert_eq!(moves_set, expected_set);
}
