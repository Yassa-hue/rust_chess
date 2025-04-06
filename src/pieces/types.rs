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

#[derive(Debug, Clone, Copy)]
pub struct Position {
  x: usize,
  y: usize,
}

impl Position {
  pub fn new(x: usize, y: usize) -> Self {
    Position { x, y }
  }

  pub fn x(&self) -> usize {
    self.x
  }

  pub fn y(&self) -> usize {
    self.y
  }

  pub fn from_str(position: &str) -> Self {
    let chars: Vec<char> = position.chars().collect();
    let x = chars[0].to_digit(10).unwrap() as usize - 1;
    let y = chars[1].to_digit(10).unwrap() as usize - 1;

    Position::new(x, y)
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
  AppliableMultiple(Vec<(i32, i32)>),
}

impl MoveOffsets {
  pub fn new_appliable_once(offsets: Vec<(i32, i32)>) -> Self {
    MoveOffsets::AppliableOnce(offsets)
  }

  pub fn new_appliable_multiple(offsets: Vec<(i32, i32)>) -> Self {
    MoveOffsets::AppliableMultiple(offsets)
  }

  pub fn apply_offsets(&self, current_position: Position) -> Vec<Position> {
    match self {
      MoveOffsets::AppliableOnce(offsets) => offsets
        .iter()
        .filter_map(|offset| current_position + *offset)
        .collect(),
      MoveOffsets::AppliableMultiple(offsets) => {
        let mut positions = vec![];
        for offset in offsets {
          let mut current_position = current_position;
          loop {
            match current_position + *offset {
              Some(new_position) => {
                positions.push(new_position);
                current_position = new_position;
              }
              None => break,
            }
          }
        }
        positions
      }
    }
  }
}
