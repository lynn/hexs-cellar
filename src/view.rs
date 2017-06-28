use grid;
use pancurses;
use pancurses::{Window, Attribute, Attributes, ColorPair, ToChtype};
use memory;
use sprite;
use sprite::{Sprite, Color};
use geometry::*;
use util::{self, pick};
use world::World;
use log::Log;
use item::Inventory;
use tile::Tile;


fn color(color: Color) -> Attributes {
    match color {
        Color::Navy   => (Attributes::new() | ColorPair(1)),
        Color::Green  => (Attributes::new() | ColorPair(2)),
        Color::Teal   => (Attributes::new() | ColorPair(3)),
        Color::Maroon => (Attributes::new() | ColorPair(4)),
        Color::Purple => (Attributes::new() | ColorPair(5)),
        Color::Brown  => (Attributes::new() | ColorPair(6)),
        Color::Gray   => (Attributes::new() | ColorPair(0)),
        Color::Dark   => (Attribute::Bold   | ColorPair(7)),
        Color::Blue   => (Attribute::Bold   | ColorPair(1)),
        Color::Lime   => (Attribute::Bold   | ColorPair(2)),
        Color::Aqua   => (Attribute::Bold   | ColorPair(3)),
        Color::Red    => (Attribute::Bold   | ColorPair(4)),
        Color::Pink   => (Attribute::Bold   | ColorPair(5)),
        Color::Yellow => (Attribute::Bold   | ColorPair(6)),
        Color::White  => (Attribute::Bold   | ColorPair(0)),
    }
}

fn cell(sprite: Sprite) -> pancurses::chtype {
    sprite.character.to_chtype() | pancurses::chtype::from(color(*pick(sprite.color)))
}

pub fn initialize() -> Window {
    let terminal = pancurses::initscr();
    pancurses::cbreak();
    pancurses::noecho();
    pancurses::curs_set(0);

    pancurses::start_color();
    pancurses::init_pair(0, pancurses::COLOR_WHITE,   pancurses::COLOR_BLACK);
    pancurses::init_pair(1, pancurses::COLOR_BLUE,    pancurses::COLOR_BLACK);
    pancurses::init_pair(2, pancurses::COLOR_GREEN,   pancurses::COLOR_BLACK);
    pancurses::init_pair(3, pancurses::COLOR_CYAN,    pancurses::COLOR_BLACK);
    pancurses::init_pair(4, pancurses::COLOR_RED,     pancurses::COLOR_BLACK);
    pancurses::init_pair(5, pancurses::COLOR_MAGENTA, pancurses::COLOR_BLACK);
    pancurses::init_pair(6, pancurses::COLOR_YELLOW,  pancurses::COLOR_BLACK);
    pancurses::init_pair(7, pancurses::COLOR_BLACK,   pancurses::COLOR_BLACK);

    terminal
}

// TODO: write an actual dang view

pub fn draw(term: &Window, world: &World) {
    term.erase(); // clear back-buffer

    if world.player.show_ram {
        draw_ram(term, &world);
    } else {
        draw_status(term, &world);
    }

    draw_board(term, &world);
    draw_messages(term, &world.log);

    term.refresh();
}

fn draw_board(term: &Window, world: &World) {
    // TODO: depend on terminal size; don't hardcode lengths/alignments
    term.attrset(Attributes::new());

    draw_border(term, world);

    let level = world.player.current_level(&world.dungeon);
    for row in 0..grid::HEIGHT as i32 {
        term.mv(row + 4, 30);
        for col in 0..grid::WIDTH as i32 {
            let position = Point(col, row);
            let sprite =
                if level.known_tiles.contains(&position) {
                    let visible = world.player.visible.contains(&position);
                    level.sprite_at(position, &world).darken(!visible)
                } else {
                    sprite::HIDDEN
                };
            term.addch(cell(sprite));
        }
    }
}

fn draw_border(term: &Window, world: &World) {
    // TODO: depend on terminal size; don't hardcode lengths/alignments

    let border = Sprite {
        character: ' ',
        color: Tile::Wall.sprite(world).color
    };

    for position in grid::RECTANGLE.grow(1) {
        let Point(col, row) = position;
        term.mvaddch(row + 4, col + 30,
            cell(border.darken(!world.player.visible.contains(&position)))
                | pancurses::chtype::from(Attribute::Reverse));
    }
}

fn draw_messages(term: &Window, log: &Log) {
    // TODO: depend on terminal size; don't hardcode lengths/alignments;
    //       visually group messages by turn (underscore separators? color?);
    //       maybe group similar messages; add linewrapping if needed
    term.attrset(Attributes::new());
    for (i, &(_, ref message)) in log.recent_messages().iter().take(6).enumerate() {
        term.mvaddstr(23 - i as i32, 0, &message);
    }
}

fn draw_status(term: &Window, world: &World) {
    draw_inventory(term, &world.player.inventory);
}

fn draw_inventory(term: &Window, inventory: &Inventory) {
    // TODO: depend on terminal size; don't hardcode lengths/alignments
    // TODO: abbreviate long inventory slot descriptions
    for (index, slot) in inventory.slots.iter().enumerate() {
        let index_color = if !slot.is_empty() {
            Color::Blue
        } else {
            Color::Dark
        };
        term.attrset(color(index_color));
        term.mvaddstr(index as i32 + 4, 0, &(index+1).to_string());

        let text_color = if slot.is_cursed() {
            if slot.is_equipped() {Color::Red} else {Color::Maroon}
        } else if !slot.is_empty() {
            if slot.is_equipped() {Color::White} else {Color::Gray}
        } else {
            Color::Dark
        };
        term.attrset(color(text_color));

        if slot.is_cursed() {
            term.addstr(" cursed");
        }

        if slot.is_enchanted() {
            term.addstr(" enchanted");
        }

        term.addch(' ');
        let name = match slot.get_item() {
            Some(item) => item.name(),
            None => "nothing"
        };
        term.addstr(name);

        if slot.is_equipped() {
            term.addstr("(equipped)");
        }
    }
}

fn draw_ram(term: &Window, world: &World) {
    for i in 0x00..0x40 {
        term.attrset(color(if i == world.player.selected { Color::Red } else { Color::Gray }));
        let hex = format!("{:02x}", memory::peek(world, i));
        term.mvaddstr(i as i32 / 8 + 4, i as i32 % 8 * 3 + 3, &*hex);
    }

    term.mvaddstr(13, 3, util::address_name(world.player.selected));

    let name = memory::player_name(world);
    term.mvaddstr(14, 3, &*name);
}
