mod colour;
mod cell;
mod board;

use std::fmt;


use self::colour::Colour;
use self::board::Board;

#[derive(Debug)]
pub struct Game {
  board: Board,
  turn: usize,
  colour_turn: Colour
}

impl fmt::Display for Game {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.board)
  }
}

impl Game {
  pub fn new() -> Self {
    Game{
      board: Board::new(),
      turn: 0,
      colour_turn: Colour::Blue
    }
  }
}



