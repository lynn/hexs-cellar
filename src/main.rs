#![allow(dead_code)]
extern crate rand;
extern crate rustty;
use rustty::{Event, Terminal};
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

use geometry::Point;
use player::Player;

fn main() {
    let mut terminal = Terminal::new().unwrap();

    let mut dungeon = dungeon::build().unwrap_or_else(|e| {
        writeln!(std::io::stderr(), "{}", e).unwrap();
        std::process::exit(1)
    });

    let mut player = Player::new(&dungeon);

    loop {
        view::draw_level(&mut terminal, &dungeon, &player);
        terminal.swap_buffers().unwrap();
        if let Some(Event::Key(key)) = terminal.get_event(Duration::from_secs(99999)).unwrap() {
            match key {
                'q' => break,
                '<' => player.try_stairs_up(&dungeon),
                '>' => player.try_stairs_down(&dungeon),
                _ => {
                    // try movement commands
                    if let Some(step_direction) = key_to_direction(key) {
                        player.step(&mut dungeon, step_direction)
                    }
                }
            }
        }
    }
}

fn key_to_direction(key: char) -> Option<Point> {
    match key {
        'y' | '7' => Some(Point(-1, -1)),
        'k' | '8' => Some(Point(0, -1)),
        'u' | '9' => Some(Point(1, -1)),
        'h' | '4' => Some(Point(-1, 0)),
        '.' | '5' | 's' => Some(Point(0, 0)),
        'l' | '6' => Some(Point(1, 0)),
        'b' | '1' => Some(Point(-1, 1)),
        'j' | '2' => Some(Point(0, 1)),
        'n' | '3' => Some(Point(1, 1)),
        _ => None
    }
}
