#![allow(dead_code)]
extern crate ncurses;
extern crate rand;
use ncurses::*;

mod addr;
mod byte;
mod dungeon;
mod item;
mod monster;
mod player;
mod point;
mod spell;
mod sprite;
mod tile;
mod timer;

fn main() {
    initscr();
    printw("Hello, world!");
    refresh();
    getch();
    endwin();
}
