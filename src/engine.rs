extern crate ncurses;

use ncurses::initscr;
use ncurses::noecho;
use ncurses::curs_set;
use ncurses::mvaddch;
use ncurses::getch;
use ncurses::clear;
use ncurses::refresh;
use ncurses::endwin;


use ncurses::CURSOR_VISIBILITY;

use crate::model::Entity;
use crate::player::handle_input;

pub fn curses_setup() {
  initscr();
  noecho();
  curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE).unwrap();
}

pub fn game_loop(player: &mut Entity) {
  loop {

    let ch = getch();
    if ch == 'q' as i32 {
      break;
    }

    handle_input(ch, player);

    clear();
    mvaddch(player.pos.y, player.pos.x, player.ch as u32);
    refresh();
  }
}

pub fn close_game() {
  endwin();
}