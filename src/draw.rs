use ncurses::mvaddch;
use ncurses::clear;

use crate::model::Entity;
use crate::map::Map;

pub fn draw_entity(entity: &Entity) {
    mvaddch(entity.pos.y as i32, entity.pos.x as i32, entity.ch as u32);
}

pub fn draw_map(map: &Map) {
   for y in 0..map.height() {
    for x in 0..map.width() {
      let uy = y as usize;
      let ux = x as usize;
      let tiles = map.tiles();
      mvaddch(uy as i32, ux as i32, tiles[uy][ux].ch as u32);
    }
  }
}

pub fn draw_everything(map: &Map, player: &Entity) {
  clear();
  draw_map(map);
  draw_entity(player);
}