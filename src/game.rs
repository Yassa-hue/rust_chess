use crate::chessboard::Chessboard;
use crate::pieces::types::{Color, Position};

pub struct Game {
  player_color: Color,
  chessboard: Chessboard,
}

impl Game {
  pub fn new(player_color: Color) -> Self {
    let chessboard = Chessboard::new();
    Game {
      player_color,
      chessboard,
    }
  }

  pub fn play(
    &mut self,
    piece_position: Position,
    target_position: Position,
  ) -> Result<(), String> {
    match self
      .chessboard
      .move_piece(piece_position, target_position, self.player_color)
    {
      Ok(_) => {
        self.player_color = self.player_color.next();
        Ok(())
      }
      Err(e) => Err(e),
    }
  }

  pub fn chessboard(&self) -> &Chessboard {
    &self.chessboard
  }

  pub fn player_color(&self) -> Color {
    self.player_color
  }
}
