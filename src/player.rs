use crate::model::Position;
use crate::model::Entity;

pub fn create_player(pos: &Position) -> Entity {
  let entity = Entity{
    pos: Position{x: pos.x, y: pos.y},
    ch: '@' as i32
  };
  return entity;
}


pub fn handle_input(input: i32, player: &mut Entity) {
  
  match std::char::from_u32(input as u32) {
    Some('k') => {
      player.pos.y = player.pos.y - 1
    },
    Some('j') => {
      player.pos.y = player.pos.y + 1
    },
    Some('h') => {
      player.pos.x = player.pos.x - 1
    },
    Some('l') => {
      player.pos.x = player.pos.x + 1
    },
    _ => println!("not valid direction")
  }
}
