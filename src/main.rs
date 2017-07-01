#![allow(dead_code)]
extern crate rand;
extern crate pancurses;

use pancurses::{Window, Input};

mod byte;
mod dungeon;
mod fov;
mod geometry;
mod grid;
mod item;
mod log;
mod memory;
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

    let terminal = view::initialize();

    loop {
        view::draw(&terminal, &world);

        let took_turn = match get_key(&terminal, &world) {
            'q' => break,
            ' ' => { world.player.show_ram = !world.player.show_ram; false },
            '<' => { world.player.try_stairs(&mut world.log, &mut world.dungeon, Stairs::Up); true },
            '>' => { world.player.try_stairs(&mut world.log, &mut world.dungeon, Stairs::Down); true },
            ',' | 'g' => world.player.pick_up_item(&mut world.log, &mut world.dungeon),
            'd' => match item_prompt(&terminal, &mut world, "Drop") {
                Some(index) => world.player.drop_item(&mut world.log, &mut world.dungeon, index),
                None => false
            },
            'a' => match item_prompt(&terminal, &mut world, "Use") {
                Some(index) => world.player.use_item(&mut world.log, &mut world.dungeon, index),
                None => false
            },
            // debug commands
            '[' => { world.player.selected = (world.player.selected + 0x3F) % 0x40; false },
            ']' => { world.player.selected = (world.player.selected + 0x01) % 0x40; false },
            '#' => {
                if let Some(b) = byte_prompt(&terminal, &mut world) {
                    let a = world.player.selected;
                    memory::poke(&mut world, a, b);
                }
                false
            },

            key => {
                // try movement commands
                if let Some(step_direction) = key_to_direction(key) {
                    world.player.step(&mut world.log, &mut world.dungeon, step_direction)
                } else {
                    false
                }
            }
        };

        if took_turn {
            monster::take_turns(&mut world);
            world.log.end_turn()
        }
    };

    pancurses::endwin();
}

// get a key and handle window resize events
fn get_key(terminal: &Window, world: &World) -> char {
    loop {
        match terminal.getch() {
            Some(Input::Character(c)) => return c,
            Some(Input::KeyResize) => {
                terminal.clearok(true);
                view::draw(terminal, world);
            }
            _ => {}
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

fn item_prompt(terminal: &Window, world: &mut World, verb: &str) -> Option<BitNumber>
{
    item_prompt_rec(terminal, world, format!("{} which item?", verb), false)
}

fn item_prompt_rec(terminal: &Window, world: &mut World,
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

fn byte_prompt(terminal: &Window, world: &mut World) -> Option<u8> {
    nibble_prompt(terminal, world, String::from("High nibble:")).and_then(|h| {
        nibble_prompt(terminal, world, String::from("Low nibble: ")).and_then(|l| {
            Some(h << 4 | l)
        })
    })
}

fn nibble_prompt(terminal: &Window, world: &mut World, prompt: String) -> Option<u8>
{
    char_prompt(terminal, world, &prompt).and_then(|key| {
        match key {
            '0' => Some(0x0), '1' => Some(0x1), '2' => Some(0x2), '3' => Some(0x3),
            '4' => Some(0x4), '5' => Some(0x5), '6' => Some(0x6), '7' => Some(0x7),
            '8' => Some(0x8), '9' => Some(0x9), 'a' => Some(0xa), 'b' => Some(0xb),
            'c' => Some(0xc), 'd' => Some(0xd), 'e' => Some(0xe), 'f' => Some(0xf),
            _ => nibble_prompt(terminal, world, prompt),
        }
    })
}

fn char_prompt(terminal: &Window, world: &mut World, prompt: &str) -> Option<char> {
    world.log.tell(String::from(prompt));
    view::draw(terminal, world);

    match get_key(terminal, world) {
        '\x1b' => {
            // escape key -- quit out of prompt
            world.log.extend_message(" Okay, then.");
            return None
        },
        key => {
            world.log.extend_message(&format!(" {}", key));
            return Some(key)
        }
    }
}
