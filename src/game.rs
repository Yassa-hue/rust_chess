use crate::chessboard::Chessboard;
use crate::movables::{Color, Movable, Position};

pub struct Game {
    player_color: Color,
    chessboard: Chessboard,
}

impl Game {
    pub fn new(player_color: Color) -> Self {
        let mut chessboard = Chessboard::new();
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
        match self.chessboard.move_piece(piece_position, target_position) {
            Ok(_) => {
                self.player_color = self.player_color.next();
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
