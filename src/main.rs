mod board;
use board::Game;
use std::io;
use std::io::Write;
use std::thread::*;

extern crate rand;
use self::rand::Rng;

fn main() {
    let mut game = Game::new();
    println!("{}", game.get_board());

    while !game.check() {
      println!("Where would you like to place your mark?");

      let mut loc = String::new();
      io::stdin().read_line(&mut loc)
          .ok()
          .expect("Failed to read line");
      let game_loc = Game::parse_location(&loc);
      game_loc
          .expect("Invalid location specification. Column should follow row, \
                   for example A1.");

      game.mark(game_loc.unwrap(), board::BoardState::X);
      println!("");

      println!("{}", game.get_board());
      println!("Mark placed at {}", loc);

      // AI
      print!("AI is deliberating");
      io::stdout().flush().ok().expect("Failed to flush");
      for i in 0 .. rand::thread_rng().gen_range(3, 5) {
        sleep_ms(1000);
        print!(".");
        io::stdout().flush().ok().expect("Failed to flush");
      }
      print!("\n");

      game.take_turn();
      println!("{}", game.get_board());
      println!("AI placed their mark.");
    }

    println!("{} has won!", game.get_winner());
}
