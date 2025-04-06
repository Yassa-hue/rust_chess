mod chessboard;
mod game;
mod pieces;
mod presenters;

fn main() {
  print!("Chess Game");

  print!("Initializing...");
  let mut game = game::Game::new(pieces::Color::White);
  println!("Game initialized.");

  println!("Starting game loop...");
  loop {
    println!("{}", game);

    let mut input = String::new();
    println!("Enter your move (e.g., e2 e4): ");
    std::io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let positions: Vec<&str> = input.trim().split_whitespace().collect();

    if positions.len() != 2 {
      println!("Invalid input. Please enter two positions.");
      continue;
    }

    let start_pos = pieces::Position::from_str(positions[0]);
    let end_pos = pieces::Position::from_str(positions[1]);

    match game.play(start_pos, end_pos) {
      Ok(_) => println!("Move successful!"),
      Err(e) => println!("Error: {}", e),
    }
  }
}
