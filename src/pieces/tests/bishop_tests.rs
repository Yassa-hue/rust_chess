use crate::pieces::traits::Movable;
use crate::pieces::{Bishop, Color, Position};

#[test]
fn test_bishop_moves_center() {
  let bishop = Bishop::new(Color::White);
  let position = Position::new(3, 3); // d4

  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  for file in 0..8 {
    for rank in 0..8 {
      let target = Position::new(file, rank);
      if position != target && bishop.can_reach(position, target, &can_step_into) {
        moves.push(target);
      }
    }
  }

  let expected: Vec<Position> = vec![
    // ↗ northeast
    Position::new(4, 4),
    Position::new(5, 5),
    Position::new(6, 6),
    Position::new(7, 7),
    // ↘ southeast
    Position::new(4, 2),
    Position::new(5, 1),
    Position::new(6, 0),
    // ↖ northwest
    Position::new(2, 4),
    Position::new(1, 5),
    Position::new(0, 6),
    // ↙ southwest
    Position::new(2, 2),
    Position::new(1, 1),
    Position::new(0, 0),
  ];

  moves.sort();
  let mut expected_sorted = expected.clone();
  expected_sorted.sort();

  assert_eq!(moves, expected_sorted);
}

#[test]
fn test_bishop_moves_from_corner() {
  let bishop = Bishop::new(Color::White);
  let position = Position::new(0, 0); // a1

  let can_step_into = |_pos: Position| true;

  let mut moves = Vec::new();

  for file in 0..8 {
    for rank in 0..8 {
      let target = Position::new(file, rank);
      if position != target && bishop.can_reach(position, target, &can_step_into) {
        moves.push(target);
      }
    }
  }

  let expected = vec![
    Position::new(1, 1),
    Position::new(2, 2),
    Position::new(3, 3),
    Position::new(4, 4),
    Position::new(5, 5),
    Position::new(6, 6),
    Position::new(7, 7),
  ];

  moves.sort();
  let mut expected_sorted = expected.clone();
  expected_sorted.sort();

  assert_eq!(moves, expected_sorted);
}
