use crate::pieces::traits::Movable;
use crate::pieces::{Color, Knight, Position};

#[test]
fn test_knight_moves_center() {
  let knight = Knight::new(Color::White);
  let pos = Position::new(4, 4);
  let moves = knight.get_valid_moves(pos);

  let expected = vec![
    Position::new(6, 5),
    Position::new(6, 3),
    Position::new(2, 5),
    Position::new(2, 3),
    Position::new(5, 6),
    Position::new(5, 2),
    Position::new(3, 6),
    Position::new(3, 2),
  ];

  assert_eq!(moves, expected);
}

#[test]
fn test_knight_moves_near_edge() {
  let knight = Knight::new(Color::Black);
  let pos = Position::new(0, 0);
  let moves = knight.get_valid_moves(pos);

  let expected = vec![Position::new(2, 1), Position::new(1, 2)];

  assert_eq!(moves, expected);
}
