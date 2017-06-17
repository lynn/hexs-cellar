#![allow(dead_code)]
extern crate ncurses;
extern crate rand;
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
