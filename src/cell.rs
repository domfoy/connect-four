use std::fmt;

use super::colour::Colour;

#[derive(Debug)]
pub(in super) struct Cell {
  pub colour: Option<Colour>
}

impl fmt::Display for Cell{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.colour {
      Some(ref c) => write!(f, "{}", c),
      None => write!(f, "."),
    }
  }
}

impl Cell {
  pub fn new() -> Self {
    Cell{colour: None}
  }

  pub fn is_free(&self) -> bool {
    self.colour == Option::None
  }
}
