#![allow(dead_code)]
extern crate ncurses;
extern crate rand;
use ncurses::*;
use std::io::Write;

mod addr;
mod byte;
mod dungeon;
mod grid;
mod item;
mod monster;
mod player;
mod point;
mod spell;
mod sprite;
mod tile;
mod timer;
mod util;

fn main() {
    match dungeon::build() {
        Ok(_) => {}
        Err(e) => {
            writeln!(std::io::stderr(), "{}", e).unwrap();
            std::process::exit(1)
        }
    }

    initscr();
    raw();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    let result = std::panic::catch_unwind(|| {
        printw("Hello, world!");
        refresh();
        getch();
    });
    endwin();

    std::process::exit(match result {
        Ok(_) => 0,
        Err(e) => {
            if let Some(s) = e.downcast_ref::<&'static str>() {
                writeln!(std::io::stderr(), "{}", s).unwrap();
            }
            if let Some(s) = e.downcast_ref::<std::io::Error>() {
                writeln!(std::io::stderr(), "{}", s).unwrap();
            }
            1
        }
    });
}
