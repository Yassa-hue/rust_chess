use crate::pieces::types::color::Color;
use crate::pieces::types::move_direction::{MoveDirection, MoveOffsets};
use crate::pieces::types::position::Position;

#[test]
fn test_color_next() {
  let white = Color::White;
  let black = Color::Black;

  assert_eq!(white.next(), Color::Black);
  assert_eq!(black.next(), Color::White);
}

#[test]
fn test_position_new() {
  let pos = Position::new(3, 4).unwrap();
  assert_eq!(pos.x(), 3);
  assert_eq!(pos.y(), 4);
}

#[test]
fn test_position_from_str() {
  let pos = Position::from_str("3A").unwrap();
  assert_eq!(pos.x(), 2); // 3 - 1 = 2
  assert_eq!(pos.y(), 0); // A - 1 = 0
}

#[test]
fn test_position_add() {
  let pos = Position::new(3, 4).unwrap();

  // Test adding valid offset
  let new_pos = pos + (2, 2);
  assert!(new_pos.is_some());
  let new_pos = new_pos.unwrap();
  assert_eq!(new_pos.x(), 5);
  assert_eq!(new_pos.y(), 6);

  // Test adding invalid offset (out of bounds)
  let new_pos = pos + (10, 10);
  assert!(new_pos.is_none());
}

#[test]
fn test_appliable_once_valid_move() {
  let move_directions =
    vec![MoveDirection::KnightUpRight, MoveDirection::KnightDownRight];
  let move_offsets = MoveOffsets::new_appliable_once(move_directions);
  let current = Position::new(4, 4).unwrap();
  let target = Position::new(6, 5).unwrap(); // KnightUpRight

  let path = move_offsets.construct_path(current, target);
  assert_eq!(path, Some(vec![target]));
}

#[test]
fn test_appliable_once_invalid_move() {
  let offsets =
    vec![MoveDirection::KnightUpRight, MoveDirection::KnightDownLeft];
  let move_offsets = MoveOffsets::new_appliable_once(offsets);
  let current = Position::new(4, 4).unwrap();
  let target = Position::new(6, 6).unwrap(); // Not a valid knight move

  let path = move_offsets.construct_path(current, target);
  assert_eq!(path, None);
}

#[test]
fn test_appliable_multiple_valid_straight_line() {
  let offsets = vec![MoveDirection::Right]; // moving right
  let move_offsets = MoveOffsets::new_appliable_multiple(offsets);
  let current = Position::new(3, 3).unwrap();
  let target = Position::new(3, 6).unwrap();

  let path = move_offsets.construct_path(current, target);
  assert_eq!(
    path,
    Some(vec![
      Position::new(3, 4).unwrap(),
      Position::new(3, 5).unwrap(),
      Position::new(3, 6).unwrap(),
    ])
  );
}

#[test]
fn test_appliable_multiple_valid_diagonal() {
  let offsets = vec![MoveDirection::DownRight];
  let move_offsets = MoveOffsets::new_appliable_multiple(offsets);
  let current = Position::new(2, 2).unwrap();
  let target = Position::new(5, 5).unwrap();

  let path = move_offsets.construct_path(current, target);
  assert_eq!(
    path,
    Some(vec![
      Position::new(3, 3).unwrap(),
      Position::new(4, 4).unwrap(),
      Position::new(5, 5).unwrap(),
    ])
  );
}

#[test]
fn test_appliable_multiple_not_on_path() {
  let offsets = vec![MoveDirection::Down]; // moving down only
  let move_offsets = MoveOffsets::new_appliable_multiple(offsets);
  let current = Position::new(4, 4).unwrap();
  let target = Position::new(6, 6).unwrap(); // diagonal not reachable with Down

  let path = move_offsets.construct_path(current, target);
  assert_eq!(path, None);
}

#[test]
fn test_appliable_multiple_same_position() {
  let offsets = vec![MoveDirection::Down];
  let move_offsets = MoveOffsets::new_appliable_multiple(offsets);
  let current = Position::new(4, 4).unwrap();
  let target = Position::new(4, 4).unwrap(); // same as current

  let path = move_offsets.construct_path(current, target);
  assert_eq!(path, None);
}
