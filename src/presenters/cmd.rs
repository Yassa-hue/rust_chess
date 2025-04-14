use std::fmt;

use crate::game::Game;
use crate::{
  chessboard::Chessboard,
  pieces::{Bishop, Color, King, Knight, Pawn, Piece, Queen, Rook},
};

use super::Presenter;

impl Presenter for Pawn {
  fn render(&self) {
    match self.color() {
      Color::White => print!("P "),
      Color::Black => print!("p "),
    }
  }
}

impl Presenter for Bishop {
  fn render(&self) {
    match self.color() {
      Color::White => print!("B "),
      Color::Black => print!("b "),
    }
  }
}

impl Presenter for Knight {
  fn render(&self) {
    match self.color() {
      Color::White => print!("N "),
      Color::Black => print!("n "),
    }
  }
}

impl Presenter for Rook {
  fn render(&self) {
    match self.color() {
      Color::White => print!("R "),
      Color::Black => print!("r "),
    }
  }
}

impl Presenter for Queen {
  fn render(&self) {
    match self.color() {
      Color::White => print!("Q "),
      Color::Black => print!("q "),
    }
  }
}

impl Presenter for King {
  fn render(&self) {
    match self.color() {
      Color::White => print!("K "),
      Color::Black => print!("k "),
    }
  }
}

impl Presenter for Chessboard {
  fn render(&self) {
    for (i, row) in self.chessboard().iter().enumerate() {
      print!("{:3} ", 8 - i); // Add row numbers (123..)
      for square in row.iter() {
        match square {
          Some(piece) => {
            piece.render();
          }
          None => {
            print!(". "); // Empty square
          }
        }
      }
      println!(); // Move to the next line after each row
    }
    print!("    ");
    for c in b'a'..=b'h' {
      print!("{} ", c as char); // Print column labels (a-h)
    }
    println!(); // Newline for column labels

    print!("White dead pieces: ");
    self.white_dead_pieces().iter().for_each(|p| p.render());
    println!();
    print!("Black dead pieces: ");
    self.black_dead_pieces().iter().for_each(|p| p.render());
    println!();
  }
}

impl Presenter for Color {
  fn render(&self) {
    match self {
      Color::White => print!("White"),
      Color::Black => print!("Black"),
    }
  }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Color::White => write!(f, "White"),
      Color::Black => write!(f, "Black"),
    }
  }
}

impl Presenter for Game {
  fn render(&self) {
    println!("Current player: {}", self.player_color());
    self.chessboard().render(); // Render the chessboard
  }
}
