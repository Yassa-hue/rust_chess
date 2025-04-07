use crate::pieces::traits::Movable;
use crate::pieces::{Bishop, Color, Position};

#[test]
fn test_bishop_moves_center() {
  let bishop = Bishop::new(Color::White);
  let position = Position::new(3, 3); // d4

  let moves = bishop.get_valid_moves(position);
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

  assert_eq!(moves, expected);
}

#[test]
fn test_bishop_moves_from_corner() {
  let bishop = Bishop::new(Color::White);
  let position = Position::new(0, 0); // a1

  let moves = bishop.get_valid_moves(position);
  let expected = vec![
    Position::new(1, 1),
    Position::new(2, 2),
    Position::new(3, 3),
    Position::new(4, 4),
    Position::new(5, 5),
    Position::new(6, 6),
    Position::new(7, 7),
  ];

  assert_eq!(moves, expected);
}
