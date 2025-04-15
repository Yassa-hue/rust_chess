use crate::pieces::types::{BOARD_SIZE, position::Position};

#[derive(PartialEq, Debug)]
pub struct MoveDirectionOffset {
  pub dx: i32,
  pub dy: i32,
}

impl std::ops::Add<MoveDirectionOffset> for Position {
  type Output = Option<Position>;

  fn add(self, other: MoveDirectionOffset) -> Option<Position> {
    let new_x = self.x() as i32 + other.dx;
    let new_y = self.y() as i32 + other.dy;

    if new_x < 0 || new_y < 0 {
      return None;
    }

    if new_x >= BOARD_SIZE as i32 || new_y >= BOARD_SIZE as i32 {
      return None;
    }

    Position::new(new_x as usize, new_y as usize)
      .map_err(|_| ())
      .ok()
  }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MoveDirection {
  Up,
  Down,
  Left,
  Right,
  UpLeft,
  UpRight,
  DownLeft,
  DownRight,
  KnightUpLeft,
  KnightUpRight,
  KnightDownLeft,
  KnightDownRight,
  KnightLeftUp,
  KnightLeftDown,
  KnightRightUp,
  KnightRightDown,
}

impl MoveDirection {
  pub fn to_offset(&self) -> MoveDirectionOffset {
    match self {
      MoveDirection::Up => MoveDirectionOffset { dx: -1, dy: 0 },
      MoveDirection::Down => MoveDirectionOffset { dx: 1, dy: 0 },
      MoveDirection::Left => MoveDirectionOffset { dx: 0, dy: -1 },
      MoveDirection::Right => MoveDirectionOffset { dx: 0, dy: 1 },
      MoveDirection::UpLeft => MoveDirectionOffset { dx: -1, dy: -1 },
      MoveDirection::UpRight => MoveDirectionOffset { dx: -1, dy: 1 },
      MoveDirection::DownLeft => MoveDirectionOffset { dx: 1, dy: -1 },
      MoveDirection::DownRight => MoveDirectionOffset { dx: 1, dy: 1 },
      MoveDirection::KnightUpLeft => MoveDirectionOffset { dx: -2, dy: -1 },
      MoveDirection::KnightUpRight => MoveDirectionOffset { dx: -2, dy: 1 },
      MoveDirection::KnightDownLeft => MoveDirectionOffset { dx: 2, dy: -1 },
      MoveDirection::KnightDownRight => MoveDirectionOffset { dx: 2, dy: 1 },
      MoveDirection::KnightLeftUp => MoveDirectionOffset { dx: -1, dy: -2 },
      MoveDirection::KnightLeftDown => MoveDirectionOffset { dx: 1, dy: -2 },
      MoveDirection::KnightRightUp => MoveDirectionOffset { dx: -1, dy: 2 },
      MoveDirection::KnightRightDown => MoveDirectionOffset { dx: 1, dy: 2 },
    }
  }

  pub fn from_offset(offset: MoveDirectionOffset) -> Option<Self> {
    match offset {
      MoveDirectionOffset { dx: -1, dy: 0 } => Some(MoveDirection::Up),
      MoveDirectionOffset { dx: 1, dy: 0 } => Some(MoveDirection::Down),
      MoveDirectionOffset { dx: 0, dy: -1 } => Some(MoveDirection::Left),
      MoveDirectionOffset { dx: 0, dy: 1 } => Some(MoveDirection::Right),
      MoveDirectionOffset { dx: -1, dy: -1 } => Some(MoveDirection::UpLeft),
      MoveDirectionOffset { dx: -1, dy: 1 } => Some(MoveDirection::UpRight),
      MoveDirectionOffset { dx: 1, dy: -1 } => Some(MoveDirection::DownLeft),
      MoveDirectionOffset { dx: 1, dy: 1 } => Some(MoveDirection::DownRight),
      MoveDirectionOffset { dx: -2, dy: -1 } => {
        Some(MoveDirection::KnightUpLeft)
      }
      MoveDirectionOffset { dx: -2, dy: 1 } => {
        Some(MoveDirection::KnightUpRight)
      }
      MoveDirectionOffset { dx: 2, dy: -1 } => {
        Some(MoveDirection::KnightDownLeft)
      }
      MoveDirectionOffset { dx: 2, dy: 1 } => {
        Some(MoveDirection::KnightDownRight)
      }
      MoveDirectionOffset { dx: -1, dy: -2 } => {
        Some(MoveDirection::KnightLeftUp)
      }
      MoveDirectionOffset { dx: 1, dy: -2 } => {
        Some(MoveDirection::KnightLeftDown)
      }
      MoveDirectionOffset { dx: -1, dy: 2 } => {
        Some(MoveDirection::KnightRightUp)
      }
      MoveDirectionOffset { dx: 1, dy: 2 } => {
        Some(MoveDirection::KnightRightDown)
      }
      _ => None,
    }
  }
}

pub enum MoveOffsets {
  AppliableOnce(Vec<MoveDirection>),
  AppliableTwice(Vec<MoveDirection>), // Only for Pawn
  AppliableMultiple(Vec<MoveDirection>),
}

impl MoveOffsets {
  pub fn new_appliable_once(offsets: Vec<MoveDirection>) -> Self {
    MoveOffsets::AppliableOnce(offsets)
  }

  pub fn new_appliable_multiple(offsets: Vec<MoveDirection>) -> Self {
    MoveOffsets::AppliableMultiple(offsets)
  }

  pub fn new_appliable_twice(offsets: Vec<MoveDirection>) -> Self {
    MoveOffsets::AppliableTwice(offsets)
  }

  pub fn construct_path(
    &self,
    current_position: Position,
    target_position: Position,
  ) -> Option<Vec<Position>> {
    match self {
      MoveOffsets::AppliableOnce(move_directions) => {
        let move_direction = MoveDirection::from_offset(MoveDirectionOffset {
          dx: target_position.x() as i32 - current_position.x() as i32,
          dy: target_position.y() as i32 - current_position.y() as i32,
        });

        match move_direction {
          Some(direction) => {
            if move_directions.contains(&direction) {
              return Some(vec![target_position]);
            } else {
              return None;
            }
          }
          None => {
            return None;
          }
        }
      }
      MoveOffsets::AppliableTwice(moving_directions) => {
        for moving_direction in moving_directions {
          let mut path = vec![];
          let mut current = current_position;

          for _ in 0..2 {
            if let Some(next) = current + moving_direction.to_offset() {
              path.push(next);
              if next == target_position {
                return Some(path);
              }
              current = next;
            } else {
              break;
            }
          }
        }
        None
      }
      MoveOffsets::AppliableMultiple(moving_directions) => {
        for moving_direction in moving_directions {
          let mut path = vec![];
          let mut current = current_position;

          while let Some(next) = current + moving_direction.to_offset() {
            path.push(next);
            if next == target_position {
              return Some(path);
            }
            current = next;
          }
        }
        None
      }
    }
  }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum SpecialMoveValidationAction {
  EnemyPieceExists,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum SpecialMove {
  EnPassant(SpecialMoveValidationAction),
}
