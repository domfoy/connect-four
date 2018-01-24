pub const BOARD_WIDTH: usize = 7;
pub const BOARD_HEIGHT: usize = 6;

use std::fmt;
use std::mem;
use std::ptr;

use super::cell::Cell;

#[derive(Debug)]
pub(in game) struct Board([[Cell; BOARD_HEIGHT]; BOARD_WIDTH]);

macro_rules! make_array {
  ($n:expr, $constructor:expr) => {{
    let mut items: [_; $n] = mem::uninitialized();
    for (_, place) in items.iter_mut().enumerate() {
        ptr::write(place, $constructor());
    }
    items
  }}
}

impl fmt::Display for Board{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let &Board(ref s) = self;

    let mut board_info = String::with_capacity(BOARD_HEIGHT * (BOARD_WIDTH + 1));
    for i in 0..BOARD_HEIGHT {
      for j in 0..BOARD_WIDTH {
        board_info.push_str(&format!("{}", s[i][j]));
      }
      board_info.push_str("\n");
    }
    f.write_str(&board_info)
  }
}

impl Board {
  pub fn new() -> Self {
    Board(unsafe {
      make_array!(BOARD_WIDTH, || make_array!(BOARD_HEIGHT, || Cell::new()))
    })
  }

  pub fn get_available_columns(&self) -> Vec<[Cell; BOARD_HEIGHT]> {
    let &Board(ref cols) = self;

    cols
      .into_iter()
      .filter(|col|
        col.into_iter()
        .any(|cell| cell.is_free())
      )
      .collect()
  }
}