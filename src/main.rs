extern crate ncurses;
use ncurses::*;

mod monster;
mod sprite;

fn main() {
    initscr();
    printw("Hello, world!");
    refresh();
    getch();
    endwin();
}
