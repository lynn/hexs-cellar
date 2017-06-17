#![allow(dead_code)]
extern crate ncurses;
use ncurses::*;

mod addr;
mod board;
mod item;
mod monster;
mod spell;
mod sprite;
mod timer;

fn main() {
    initscr();
    printw("Hello, world!");
    refresh();
    getch();
    endwin();
}
