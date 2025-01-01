mod player;
mod engine;
mod map;
mod draw;

mod model;
use model::Position;

fn main() {

  engine::curses_setup();

  let start_pos = Position{x: 10, y: 20};
  let mut player = player::create_player(&start_pos);

  let mut map = map::Map::new();
  map.create_map_tiles();

  engine::game_loop(&mut map, &mut player);
  engine::close_game();
}