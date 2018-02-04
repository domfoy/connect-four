mod colour;
mod cell;
mod board;

use std::fmt;

pub use self::colour::Colour;
use self::board::Board;

#[derive(Debug)]
pub struct Game {
  board: Board,
  turn: usize,
  is_over: bool,
  colour_turn: Colour,
}

impl fmt::Display for Game {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "turn: {} {}\n{}", self.turn, self.colour_turn, self.board)
  }
}

impl Game {
  pub fn new() -> Self {
    Game{
      board: Board::new(),
      turn: 0,
      is_over: false,
      colour_turn: Colour::Blue,
    }
  }

  pub fn is_over(&self) -> bool {
    self.is_over
  }

  pub fn get_available_cols(&self) -> Vec<usize> {
    self.board.get_available_cols()
  }

  pub fn put_piece(&mut self, col: usize) {
    let position = self.board.put_piece(col, self.colour_turn);
    self.on_turn_end(position);
  }

  pub fn winner(&self) -> Option<Colour> {
    if self.is_over() {
      Some(self.colour_turn)
    } else {
      None
    }
  }
}

impl Game {
  fn on_turn_end(&mut self, position: (usize, usize)) {
    let (i,j) = position;
    if self.board.check_win(i, j) {
      self.is_over = true;
    } else {
      self.turn += 1;
      self.colour_turn = match self.colour_turn {
        Colour::Blue => Colour::Red,
        Colour::Red => Colour::Blue,
      };
    }
  }
}
