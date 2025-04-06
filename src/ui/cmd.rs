use crate::game::Game;
use crate::pieces::Position;
use crate::ui::GameUI;
use std::io;

pub struct CmdUI;

impl GameUI for CmdUI {
  fn start_game_loop(&mut self, game: &mut Game) {
    loop {
      println!("{}", game);

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
        Ok(_) => println!("Move successful!"),
        Err(e) => println!("Error: {}", e),
      }
    }
  }
}
