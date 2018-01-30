use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
pub(in game) enum Colour {
  Red,
  Blue,
}
impl fmt::Display for Colour{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Colour::Blue => write!(f, "B"),
      &Colour::Red => write!(f, "R"),
    }
  }
}