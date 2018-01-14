use std::mem;
use std::ptr;
use std::fmt;

const BOARD_LENGTH: usize = 25;

#[derive(Debug)]
enum Colour {
  Blue,
  Red
}
impl fmt::Display for Colour{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Colour::Blue => write!(f, "B"),
      &Colour::Red => write!(f, "R"),
    }
  }
}

#[derive(Debug)]
struct Cell {
  colour: Option<Colour>
}

impl fmt::Display for Cell{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.colour {
      Some(ref c) => write!(f, "{}", c),
      None => write!(f, "."),
    }
  }
}

#[derive(Debug)]
struct Board([Cell; BOARD_LENGTH]);

impl fmt::Display for Board{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let &Board(ref s) = self;

    let mut board_info = String::with_capacity(BOARD_LENGTH + 5);
    for chunk in s.chunks(5).rev() {
      for c in chunk {
        board_info.push_str(&format!("{}", c));
      }
      board_info.push_str("\n");
    }
    f.write_str(&board_info)
  }
}

impl Board {
  // fn get_allowed_columns(&self) -> &[usize] {
  //   let Board(ref cells) = self;
  //   let allowed_cols = [];

  //   for chunk in cells.
  // }

  fn get_columns(&self) -> Vec<Vec<&Cell>> {
    let &Board(ref cells) = self;

    let mut cols = Vec::with_capacity(5);
    for offset in 0..4 {
      let mut col = Vec::with_capacity(5);
      for line in 0..4 {
        col.push(&cells[(offset + line * 5) as usize]);
      }
      cols.push(col);
    }
    cols
  }
}

#[derive(Debug)]
struct Game {
  board: Board,
  turn: usize,
  colour_turn: Colour
}

impl fmt::Display for Game {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.board)
  }
}

macro_rules! make_array {
  ($n:expr, $constructor:expr) => {{
    let mut items: [_; $n] = mem::uninitialized();
    for (_, place) in items.iter_mut().enumerate() {
        ptr::write(place, $constructor());
    }
    items
  }}
}


fn init_game() -> Game {
  Game{
    board: Board(unsafe {make_array!(BOARD_LENGTH, || Cell{colour: None})}),
    turn: 0,
    colour_turn: Colour::Blue
  }
}

fn main() {
  let game = init_game();

  let b = Cell{colour: Some(Colour::Blue)};
  println!("Hello, world!:\n{}", &game);
  println!("Hello, world!:\n{}", &game.board.get_columns());
}
