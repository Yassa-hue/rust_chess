use crate::pieces::types::BOARD_SIZE;

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
