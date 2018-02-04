use std::io;

extern crate connect_four;

use connect_four::Game;

fn main() {
  let mut game = Game::new();

  while !game.is_over() {
    println!("{}", game);

    let mut choice: Option<usize> = None;
    let available_cols = game.get_available_cols();
    while choice.is_none() {
      println!("available: {:?}", available_cols);
      println!("Select column to put piece");

      let mut input = String::new();

      io::stdin().read_line(&mut input)
        .expect("Failed to read line");

      choice = input.trim().parse::<usize>().ok();

      if let Some(selected) = choice {
        if !available_cols.iter().any(|col| selected == *col) {
          println!("Not available column {}", selected);

          choice = None;
        }
      }
    }

    let col = choice.unwrap();
    println!("You chose: {}", col);

    game.put_piece(col);
  }
  println!("{}", game);

  println!("{} won", game.winner().unwrap());
}