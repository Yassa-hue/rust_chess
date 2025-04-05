use crate::chessboard::Chessboard;
use crate::movables::Color;

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
}
