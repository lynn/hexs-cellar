#![allow(dead_code)]
extern crate rand;
extern crate rustty;
use rustty::Terminal;
use std::io::Write;
use std::time::Duration;

mod addr;
mod byte;
mod dungeon;
mod geometry;
mod grid;
mod item;
mod monster;
mod player;
mod spell;
mod sprite;
mod tile;
mod timer;
mod util;
mod view;

fn main() {
    let mut terminal = Terminal::new().unwrap();

    let dungeon = dungeon::build().unwrap_or_else(|e| {
        writeln!(std::io::stderr(), "{}", e).unwrap();
        std::process::exit(1)
    });

    view::draw_level(&mut terminal, &dungeon[0]);
    terminal.swap_buffers().unwrap();
    terminal.get_event(Duration::from_secs(99999)).unwrap();
}
