use crate::pieces::traits::Movable;
use crate::pieces::{Color, Pawn, Position};

#[test]
fn test_white_pawn_initial_moves() {
  let pawn = Pawn::new(Color::White);
  let position = Position::new(1, 4); // e2

  let moves = pawn.get_valid_moves(position);
  let expected = vec![
    Position::new(2, 4), // e3
  ];

  assert_eq!(moves, expected);
}

#[test]
fn test_black_pawn_initial_moves() {
  let pawn = Pawn::new(Color::Black);
  let position = Position::new(6, 4); // e7

  let moves = pawn.get_valid_moves(position);
  let expected = vec![
    Position::new(5, 4), // e6
  ];

  assert_eq!(moves, expected);
}

#[test]
fn test_white_pawn_after_move() {
  let pawn = Pawn::new(Color::White);
  let position = Position::new(2, 4); // e3

  let moves = pawn.get_valid_moves(position);
  let expected = vec![
    Position::new(3, 4), // e4
  ];

  assert_eq!(moves, expected);
}

#[test]
fn test_black_pawn_after_move() {
  let pawn = Pawn::new(Color::Black);
  let position = Position::new(5, 4); // e6

  let moves = pawn.get_valid_moves(position);
  let expected = vec![
    Position::new(4, 4), // e5
  ];

  assert_eq!(moves, expected);
}
