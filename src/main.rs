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
use byte::BitNumber;

fn main() {
    let mut world = World::new();
    let mut terminal = Terminal::new().unwrap();

    loop {
        terminal.clear().unwrap(); // clear back-buffer and notice terminal resize
        view::draw(&mut terminal, &world);
        terminal.swap_buffers().unwrap();
        if let Some(Event::Key(key)) = terminal.get_event(Duration::from_secs(99999)).unwrap() {
            let took_turn = match key {
                'q' => break,
                '<' => { world.player.try_stairs(&mut world.log, &mut world.dungeon, Stairs::Up); false },
                '>' => { world.player.try_stairs(&mut world.log, &mut world.dungeon, Stairs::Down); false },
                '\x1b' => { eat_escape_sequence(&mut terminal); continue }
                ',' | 'g' => world.player.pick_up_item(&mut world.log, &mut world.dungeon),
                'd' => match item_prompt(&mut terminal, &mut world, "Drop") {
                    Some(index) => world.player.drop_item(&mut world.log, &mut world.dungeon, index),
                    None => false
                },
                _ => {
                    // try movement commands
                    if let Some(step_direction) = key_to_direction(key) {
                        world.player.step(&mut world.log, &mut world.dungeon, step_direction)
                    } else {
                        false
                    }
                }
            };

            if took_turn {
                world.log.end_turn()
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

fn item_prompt(terminal: &mut Terminal, world: &mut World, verb: &str) -> Option<BitNumber>
{
    item_prompt_rec(terminal, world, format!("{} which item?", verb), false)
}

fn item_prompt_rec(terminal: &mut Terminal, world: &mut World,
    prompt: String, explained: bool) -> Option<BitNumber>
{
    use BitNumber::*;

    char_prompt(terminal, world, &prompt).and_then(|key| {
        match key {
            '1' => Some(Bit0),
            '2' => Some(Bit1),
            '3' => Some(Bit2),
            '4' => Some(Bit3),
            '5' => Some(Bit4),
            '6' => Some(Bit5),
            '7' => Some(Bit6),
            '8' => Some(Bit7),
            _ => item_prompt_rec(terminal, world,
                if explained {
                    prompt
                } else {
                    prompt + " (a digit 1-8)"
                }, true)
        }
    })
}

fn char_prompt(terminal: &mut Terminal, world: &mut World, prompt: &str) -> Option<char> {
    world.log.tell(String::from(prompt));
    loop {
        terminal.clear().unwrap();
        view::draw(terminal, world);
        terminal.swap_buffers().unwrap();
        if let Some(Event::Key(key)) = terminal.get_event(Duration::from_secs(99999)).unwrap() {
            if key == '\x1b' {
                if eat_escape_sequent(terminal) {
                    eat_escape_sequence(terminal)
                } else {
                    // quit out of prompt on single escape press.
                    world.log.extend_message("Okay, then.");
                    return None
                }
            } else {
                world.log.extend_message(&format!(" {}", key));
                return Some(key)
            }
        }
    }
}

fn eat_escape_sequent(terminal: &mut Terminal) -> bool {
    terminal.get_event(Duration::from_millis(1)).unwrap().is_some()
}

fn eat_escape_sequence(terminal: &mut Terminal) {
    while eat_escape_sequent(terminal) {}
}
