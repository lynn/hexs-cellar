use grid;
use rustty;
use rustty::{Terminal, Cell, Attr};
use rustty::ui::Painter;
use sprite;
use sprite::Sprite;
use geometry::*;
use util::pick;
use world::World;


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

pub fn draw(term: &mut Terminal, world: &World) {
    // TODO: replace this with rustty:ui stuff... actually TODO write an actual dang view
    let level = world.player.current_level(&world.dungeon);
    let blue = Cell::new('#', rustty::Color::Blue, rustty::Color::Blue, Attr::Default);
    for position in grid::RECTANGLE.grow(1) {
        let Point(col, row) = position;
        term[((col + 30) as usize, (row + 4) as usize)] = blue;
    }
    for position in grid::RECTANGLE {
        let sprite =
            if level.known_tiles.contains(&position) {
                let visible = world.player.visible.contains(&position);
                level.sprite_at(position, &world).darken(!visible)
            } else {
                sprite::HIDDEN
            };
        let Point(col, row) = position;
        term[((col + 30) as usize, (row + 4) as usize)] = cell(sprite)
    }
    term.printline(30, 18, "q to quit");
}
