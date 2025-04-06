pub mod cmd;

use crate::game::Game;

pub trait GameUI {
  fn start_game_loop(&mut self, game: &mut Game);
}
