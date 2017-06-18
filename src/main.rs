#![allow(dead_code)]
extern crate rand;
extern crate rustty;
use rustty::{Terminal, Cell, Attr, Color};
use std::io::Write;
use std::time::Duration;

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
mod view;

fn main() {
    let mut terminal = Terminal::new().unwrap();

    let dungeon = dungeon::build().unwrap_or_else(|e| {
        writeln!(std::io::stderr(), "{}", e).unwrap();
        std::process::exit(1)
    });
    // let result = std::panic::catch_unwind(|| {
        // view::draw_level(&dungeon[0]);
        terminal.clear_with_cell(
            Cell::new('@', Color::Magenta, Color::Default, Attr::Bold)
        ).unwrap();
        terminal.swap_buffers().unwrap();
        terminal.get_event(Duration::from_secs(99999)).unwrap();
    // });

    /* std::process::exit(match result {
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
    }); */
}
