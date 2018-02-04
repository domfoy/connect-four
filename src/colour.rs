use std::fmt;

#[derive(Debug)]
#[derive(PartialEq, Copy, Clone)]
pub enum Colour {
  Red,
  Blue,
}
impl fmt::Display for Colour{
  fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Colour::Blue => write!(f, "B"),
      Colour::Red => write!(f, "R"),
    }
  }
}
