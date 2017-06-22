#![allow(dead_code)]
extern crate rand;
extern crate rustty;
use rustty::{Event, Terminal};
use std::time::Duration;

mod addr;
mod byte;
mod dungeon;
mod fov;
mod geometry;
mod grid;
mod item;
mod log;
mod monster;
mod player;
mod spell;
mod sprite;
mod tile;
mod timer;
mod util;
mod view;
mod world;

use world::World;
use geometry::Point;
use tile::Stairs;

fn main() {
    let mut world = World::new();
    let mut terminal = Terminal::new().unwrap();

    loop {
        terminal.clear().unwrap(); // clear back-buffer and notice terminal resize
        view::draw(&mut terminal, &world);
        terminal.swap_buffers().unwrap();
        if let Some(Event::Key(key)) = terminal.get_event(Duration::from_secs(99999)).unwrap() {
            match key {
                'q' => break,
                '<' => world.player.try_stairs(&mut world.log, &mut world.dungeon, Stairs::Up),
                '>' => world.player.try_stairs(&mut world.log, &mut world.dungeon, Stairs::Down),
                '\x1b' => {
                    // eat escape sequences
                    while terminal.get_event(Duration::from_millis(1)).unwrap().is_some() {
                    }
                }
                _ => {
                    // try movement commands
                    if let Some(step_direction) = key_to_direction(key) {
                        let took_turn = world.player.step(&mut world.log, &mut world.dungeon, step_direction);
                        if took_turn {
                            world.log.end_turn()
                        }
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
