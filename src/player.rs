use crate::model::Position;
use crate::model::Entity;

pub fn create_player(pos: &Position) -> Entity {
  let entity = Entity{
    pos: Position{x: pos.x, y: pos.y},
    ch: '@' as i32
  };
  return entity;
}


pub fn handle_input(input: i32, player: &mut Entity) -> Position {
  let mut new_player_pos = Position{y: player.pos.y, x: player.pos.x};

  match std::char::from_u32(input as u32) {
    Some('k') => {
      new_player_pos.y = new_player_pos.y - 1
    },
    Some('j') => {
      new_player_pos.y = new_player_pos.y + 1
    },
    Some('h') => {
      new_player_pos.x = new_player_pos.x - 1
    },
    Some('l') => {
      new_player_pos.x = new_player_pos.x + 1
    },
    _ => println!("not valid direction")
  }

  new_player_pos
}
