extern crate ncurses;

use ncurses::initscr;
use ncurses::noecho;
use ncurses::curs_set;
use ncurses::getch;
use ncurses::refresh;
use ncurses::endwin;

use ncurses::CURSOR_VISIBILITY;

use crate::model::Entity;
use crate::map::Map;
use crate::player::handle_input;

use crate::draw::draw_everything;

pub fn curses_setup() {
  initscr();
  noecho();
  curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE).unwrap();
}

pub fn game_loop(map: &mut Map, player: &mut Entity) {
  loop {
    
    draw_everything(map, player);

    let ch = getch();
    if ch == 'q' as i32 {
      break;
    }

    handle_input(ch, player);
    draw_everything(map, player);

    refresh();
  }
}

pub fn close_game() {
  endwin();
}