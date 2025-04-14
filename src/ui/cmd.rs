use crate::chessboard::MoveResult;
use crate::game::Game;
use crate::pieces::Position;
use crate::presenters::Presenter;
use crate::ui::GameUI;
use std::io;

pub struct CmdUI;

impl GameUI for CmdUI {
  fn start_game_loop(&mut self, game: &mut Game) {
    loop {
      game.render();

      let mut input = String::new();
      println!("Enter your move (e.g., e2 e4): ");
      io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

      let positions: Vec<&str> = input.trim().split_whitespace().collect();

      if positions.len() != 2 {
        println!("Invalid input. Please enter two positions.");
        continue;
      }

      let start_pos = Position::from_str(positions[0]);
      let end_pos = Position::from_str(positions[1]);

      match game.play(start_pos, end_pos) {
        Ok(res) => {
          println!("Move successful!");
          match res {
            MoveResult::None => (),
            MoveResult::CanUpgradePiece => self.handle_upgrade_piece(game, end_pos),
          }
        }
        Err(e) => println!("Error: {}", e),
      }
    }
  }

  fn handle_upgrade_piece(&mut self, game: &mut Game, upgrade_position: Position) {
    println!("You can upgrade your piece!");
    println!("Enter the index of new piece type (e.g., Q for Queen, R for Rook): ");
    let mut piece_input = String::new();
    io::stdin()
      .read_line(&mut piece_input)
      .expect("Failed to read line");
    let piece_index: usize = piece_input
      .trim()
      .to_string()
      .parse()
      .expect("Invalid input");

    game.upgrade_piece(piece_index, upgrade_position);
  }
}
