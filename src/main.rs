mod game;

use game::Game;

fn main() {
  let game = Game::new();

  // let b = Cell{colour: Some(Colour::Blue)};
  println!("Hello, world!:\n{}", &game);
  // println!("Hello, world!:\n{:?}", &game.board.get_columns());
}
