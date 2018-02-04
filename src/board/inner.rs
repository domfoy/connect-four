use std::cmp;

use super::{
  Board,
  BOARD_HEIGHT,
  BOARD_WIDTH,
  LINE_LENGTH
};
use ::cell::Cell;

#[derive(Debug)]
enum LineType {
  Vertical,
  Horizontal,
  Diagonal1,
  Diagonal2
}
use self::LineType::*;

impl Board {
  pub (in super) fn check_horizontal(&self, i: usize, j: usize) -> bool {
    self.check_straight_line_win(i, j, Horizontal)
  }
  pub (in super) fn check_vertical(&self, i: usize, j: usize) -> bool {
    self.check_straight_line_win(i, j, Vertical)
  }
  pub (in super) fn check_first_diagonal1(&self, i: usize, j: usize) -> bool {
    self.check_straight_line_win(i, j, Diagonal1)
  }
  pub (in super) fn check_first_diagonal2(&self, i: usize, j: usize) -> bool {
    self.check_straight_line_win(i, j, Diagonal2)
  }
}

impl Board {
  fn check_straight_line_win<'a>(&'a self, i: usize, j: usize, line_type: LineType) -> bool {
    let &Board(ref cells) = self;
    let candidate_colour = &cells[i][j].colour;

    let offset = match line_type {
      Diagonal1 => cmp::min(i,j),
      Diagonal2 => cmp::min(j, BOARD_HEIGHT - 1 - i),
      _ => 0
    };

    let end = match line_type {
      Horizontal => BOARD_WIDTH,
      Vertical => BOARD_HEIGHT,
      Diagonal1 => cmp::min(BOARD_WIDTH + offset - j, BOARD_HEIGHT + offset - i),
      Diagonal2 => cmp::min(BOARD_WIDTH + offset - j, i + offset + 1),
    };

    let get_cell: Box<Fn(usize) -> &'a Cell> = match line_type {
      Horizontal => Box::new(|t: usize| &cells[i][t]),
      Vertical => Box::new(|t: usize| &cells[t][j]),
      Diagonal1 => Box::new(|t: usize| &cells[i - offset + t][j - offset + t]),
      Diagonal2 => Box::new(|t: usize| &cells[i + offset - t][j - offset + t]),
    };

    let mut t = 0;
    let mut candidate_line_length = 0;
    while t < end && candidate_line_length < LINE_LENGTH {
      let candidate_cell = get_cell(t);

      if candidate_cell.colour == *candidate_colour {
        candidate_line_length += 1;
      } else {
        candidate_line_length = 0;
      }

      t += 1;
    }

    candidate_line_length >= LINE_LENGTH
  }
}