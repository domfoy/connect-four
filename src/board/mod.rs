const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;
const LINE_LENGTH: usize = 4;

use std::fmt;
use std::mem;
use std::ptr;

use super::cell::Cell;
use super::colour::Colour;

mod inner;

#[derive(Debug)]
pub(in super) struct Board([[Cell; BOARD_WIDTH]; BOARD_HEIGHT]);

macro_rules! make_array {
  ($n:expr, $constructor:expr) => {{
    let mut items: [_; $n] = mem::uninitialized();
    for (_, place) in items.iter_mut().enumerate() {
        ptr::write(place, $constructor());
    }
    items
  }}
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let &Board(ref s) = self;

    let mut board_info = String::with_capacity(BOARD_HEIGHT * (BOARD_WIDTH + 1));
    for i in 0..BOARD_HEIGHT {
      for j in 0..BOARD_WIDTH {
        board_info.push_str(&format!("{}", s[BOARD_HEIGHT-1-i][j]));
      }
      board_info.push_str("\n");
    }
    f.write_str(&board_info)
  }
}

impl Board {
  pub fn new() -> Self {
    Board(unsafe {
      make_array!(BOARD_HEIGHT, || make_array!(BOARD_WIDTH, || Cell::new()))
    })
  }

  pub fn get_available_cols(&self) -> Vec<usize> {
    let mut available = Vec::with_capacity(BOARD_WIDTH);

    for j in 0..BOARD_WIDTH {
      let mut is_available = false;
      for i in 0..BOARD_HEIGHT {
        if self.0[i][j].is_free() {
          is_available = true;
          break;
        }
      }
      if is_available {
        available.push(j);
      }
    }

    available
  }

  pub fn put_piece(&mut self, col: usize, colour: Colour) -> (usize, usize) {
    let &mut Board(ref mut cells) =  self;
    let mut line = 0;
    while !cells[line][col].is_free() {line += 1};

    cells[line][col].colour = Some(colour);
    (line, col)
  }

  pub fn check_win(&self, i: usize, j: usize) -> bool {
    self.check_horizontal(i, j)
    || self.check_vertical(i, j)
    || self.check_first_diagonal1(i, j)
    || self.check_first_diagonal2(i, j)
  }
}