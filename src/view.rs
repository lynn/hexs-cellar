use grid;
use rustty;
use rustty::{Terminal, Cell, Attr};
use sprite;
use sprite::Sprite;
use geometry::*;
use dungeon::Level;
use player::Player;
use util::pick;


fn cell(sprite: Sprite) -> Cell {
    let (color, attr) = match *pick(sprite.color) {
        sprite::Color::Navy   => (rustty::Color::Blue,    Attr::Default),
        sprite::Color::Green  => (rustty::Color::Green,   Attr::Default),
        sprite::Color::Teal   => (rustty::Color::Cyan,    Attr::Default),
        sprite::Color::Maroon => (rustty::Color::Red,     Attr::Default),
        sprite::Color::Purple => (rustty::Color::Magenta, Attr::Default),
        sprite::Color::Brown  => (rustty::Color::Yellow,  Attr::Default),
        sprite::Color::Gray   => (rustty::Color::White,   Attr::Default),
        sprite::Color::Dark   => (rustty::Color::Black,   Attr::Bold),
        sprite::Color::Blue   => (rustty::Color::Blue,    Attr::Bold),
        sprite::Color::Lime   => (rustty::Color::Green,   Attr::Bold),
        sprite::Color::Aqua   => (rustty::Color::Cyan,    Attr::Bold),
        sprite::Color::Red    => (rustty::Color::Red,     Attr::Bold),
        sprite::Color::Pink   => (rustty::Color::Magenta, Attr::Bold),
        sprite::Color::Yellow => (rustty::Color::Yellow,  Attr::Bold),
        sprite::Color::White  => (rustty::Color::White,   Attr::Bold),
    };
    Cell::new(sprite.character, color, rustty::Color::Default, attr)
}

pub fn draw_level(term: &mut Terminal, level: &Level, player: &Player) {
    for position in grid::RECTANGLE {
        let sprite = level.sprite_at(position, player);
        let Point(row, col) = position;
        term[(row as usize, col as usize)] = cell(sprite)
    }
}
