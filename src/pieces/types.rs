pub const BOARD_SIZE: usize = 8;

#[derive(Debug, Clone, Copy)]
pub enum Color {
  White,
  Black,
}
impl Color {
  pub fn next(&self) -> Self {
    match self {
      Color::White => Color::Black,
      Color::Black => Color::White,
    }
  }
}
impl PartialEq for Color {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Color::White, Color::White) => true,
      (Color::Black, Color::Black) => true,
      _ => false,
    }
  }
}

#[derive(Debug, Clone, Copy, Eq, Hash)]
pub struct Position {
  x: usize,
  y: usize,
}

impl PartialOrd for Position {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Position {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    (self.x, self.y).cmp(&(other.x, other.y))
  }
}

impl Position {
  pub fn new(x: usize, y: usize) -> Result<Self, ()> {
    if x >= BOARD_SIZE || y >= BOARD_SIZE {
      return Err(());
    }
    Ok(Position { x, y })
  }

  pub fn x(&self) -> usize {
    self.x
  }

  pub fn y(&self) -> usize {
    self.y
  }

  pub fn from_str(position: &str) -> Result<Self, String> {
    if position.len() != 2 {
      return Err("Invalid position format".to_string());
    }
    let chars: Vec<char> = position.chars().collect();

    let x = chars[0].to_digit(10).unwrap() as usize - 1;
    let y = (chars[1] as u8 - b'A') as usize;

    Position::new(x, y).map_err(|_| "Position out of bounds".to_string())
  }
}

impl std::ops::Add<(i32, i32)> for Position {
  type Output = Option<Position>;

  fn add(self, other: (i32, i32)) -> Option<Position> {
    let new_x = self.x as i32 + other.0;
    let new_y = self.y as i32 + other.1;

    if new_x < 0 || new_y < 0 {
      return None;
    }

    if new_x >= BOARD_SIZE as i32 || new_y >= BOARD_SIZE as i32 {
      return None;
    }

    Some(Position {
      x: new_x as usize,
      y: new_y as usize,
    })
  }
}

impl PartialEq for Position {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}

pub enum MoveOffsets {
  AppliableOnce(Vec<(i32, i32)>),
  AppliableTwice(Vec<(i32, i32)>), // Only for Pawn
  AppliableMultiple(Vec<(i32, i32)>),
}

impl MoveOffsets {
  pub fn new_appliable_once(offsets: Vec<(i32, i32)>) -> Self {
    MoveOffsets::AppliableOnce(offsets)
  }

  pub fn new_appliable_multiple(offsets: Vec<(i32, i32)>) -> Self {
    MoveOffsets::AppliableMultiple(offsets)
  }

  pub fn new_appliable_twice(offsets: Vec<(i32, i32)>) -> Self {
    MoveOffsets::AppliableTwice(offsets)
  }

  pub fn construct_path(
    &self,
    current_position: Position,
    target_position: Position,
  ) -> Option<Vec<Position>> {
    match self {
      MoveOffsets::AppliableOnce(offsets) => {
        if offsets.contains(&(
          target_position.x as i32 - current_position.x as i32,
          target_position.y as i32 - current_position.y as i32,
        )) {
          Some(vec![target_position])
        } else {
          None
        }
      }
      MoveOffsets::AppliableTwice(offsets) => {
        for offset in offsets {
          let mut path = vec![];
          let mut current = current_position;

          for _ in 0..2 {
            if let Some(next) = current + *offset {
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
      MoveOffsets::AppliableMultiple(offsets) => {
        for offset in offsets {
          let mut path = vec![];
          let mut current = current_position;

          while let Some(next) = current + *offset {
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
