mod model;
use model::Position;

mod player;
mod engine;


fn main() {

  engine::curses_setup();

  let start_pos = Position{x: 10, y: 20};
  let mut player = player::create_player(&start_pos);

  engine::game_loop(&mut player);
  engine::close_game();
}
