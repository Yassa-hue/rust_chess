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
