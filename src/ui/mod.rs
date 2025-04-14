pub mod cmd;

use crate::game::Game;
use crate::pieces::types::Position;

pub trait GameUI {
  fn start_game_loop(&mut self, game: &mut Game);

  fn handle_upgrade_piece(&mut self, game: &mut Game, upgrade_position: Position);
}
