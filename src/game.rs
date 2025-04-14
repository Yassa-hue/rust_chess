use crate::chessboard::{Chessboard, MoveResult};
use crate::chessboard_factory::ChessboardFactory;
use crate::pieces::types::{Color, Position};

pub struct Game {
  player_color: Color,
  chessboard: Chessboard,
}

impl Game {
  pub fn new(player_color: Color) -> Self {
    let board = ChessboardFactory::standard_board();
    let chessboard = Chessboard::new(board);
    Game {
      player_color,
      chessboard,
    }
  }

  pub fn play(
    &mut self,
    piece_position: Position,
    target_position: Position,
  ) -> Result<MoveResult, String> {
    match self
      .chessboard
      .move_piece(piece_position, target_position, self.player_color)
    {
      Ok(res) => {
        self.player_color = self.player_color.next();
        Ok(res)
      }
      Err(e) => Err(e),
    }
  }

  pub fn upgrade_piece(&mut self, piece_index: usize, upgrade_position: Position) {
    self
      .chessboard
      .upgrade_piece(piece_index, self.player_color().next(), upgrade_position);
  }

  pub fn chessboard(&self) -> &Chessboard {
    &self.chessboard
  }

  pub fn player_color(&self) -> Color {
    self.player_color
  }
}
