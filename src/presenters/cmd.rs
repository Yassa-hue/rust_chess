use crate::game::Game;
use crate::{
  chessboard::Chessboard,
  pieces::{Bishop, Color, King, Knight, Movable, Pawn, Queen, Rook},
};
use std::fmt::{self, Display, Formatter};

impl Display for Pawn {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self.color() {
      Color::White => write!(f, "P"),
      Color::Black => write!(f, "p"),
    }
  }
}

impl Display for Bishop {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self.color() {
      Color::White => write!(f, "B"),
      Color::Black => write!(f, "b"),
    }
  }
}

impl Display for Knight {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self.color() {
      Color::White => write!(f, "N"),
      Color::Black => write!(f, "n"),
    }
  }
}

impl Display for Rook {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self.color() {
      Color::White => write!(f, "R"),
      Color::Black => write!(f, "r"),
    }
  }
}

impl Display for Queen {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self.color() {
      Color::White => write!(f, "Q"),
      Color::Black => write!(f, "q"),
    }
  }
}

impl Display for King {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self.color() {
      Color::White => write!(f, "K"),
      Color::Black => write!(f, "k"),
    }
  }
}

impl fmt::Display for Chessboard {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for (i, row) in self.chessboard().iter().enumerate() {
      write!(f, "{:3} ", 8 - i)?; // Add row numbers (123..)
      for square in row.iter() {
        match square {
          Some(piece) => {
            write!(f, "{} ", piece)?;
          }
          None => {
            write!(f, ". ")?;
          }
        }
      }
      writeln!(f)?;
    }
    write!(f, "    ")?;
    for c in b'a'..=b'h' {
      write!(f, "{} ", c as char)?;
    }
    writeln!(f)?;
    Ok(())
  }
}

impl Display for Color {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Color::White => write!(f, "White"),
      Color::Black => write!(f, "Black"),
    }
  }
}

impl Display for Game {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "Current player: {}", self.player_color())?;
    write!(f, "\n{}", self.chessboard())
  }
}
