use grid;
use pancurses;
use pancurses::{Window, Attribute, Attributes, ColorPair, ToChtype};
use sprite;
use sprite::Sprite;
use geometry::*;
use util::pick;
use world::World;
use log::Log;


fn cell(sprite: Sprite) -> pancurses::chtype {
    let attributes = match *pick(sprite.color) {
        sprite::Color::Navy   => (Attributes::new() | ColorPair(1)),
        sprite::Color::Green  => (Attributes::new() | ColorPair(2)),
        sprite::Color::Teal   => (Attributes::new() | ColorPair(3)),
        sprite::Color::Maroon => (Attributes::new() | ColorPair(4)),
        sprite::Color::Purple => (Attributes::new() | ColorPair(5)),
        sprite::Color::Brown  => (Attributes::new() | ColorPair(6)),
        sprite::Color::Gray   => (Attributes::new() | ColorPair(0)),
        sprite::Color::Dark   => (Attribute::Bold   | ColorPair(7)),
        sprite::Color::Blue   => (Attribute::Bold   | ColorPair(1)),
        sprite::Color::Lime   => (Attribute::Bold   | ColorPair(2)),
        sprite::Color::Aqua   => (Attribute::Bold   | ColorPair(3)),
        sprite::Color::Red    => (Attribute::Bold   | ColorPair(4)),
        sprite::Color::Pink   => (Attribute::Bold   | ColorPair(5)),
        sprite::Color::Yellow => (Attribute::Bold   | ColorPair(6)),
        sprite::Color::White  => (Attribute::Bold   | ColorPair(0)),
    };

    sprite.character.to_chtype() | pancurses::chtype::from(attributes)
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

    // NOTE: only used for drawing border; remove if we end up not needing this
    pancurses::init_pair(8, pancurses::COLOR_BLUE, pancurses::COLOR_BLUE);

    terminal
}

// TODO: write an actual dang view

pub fn draw(term: &Window, world: &World) {
    term.erase(); // clear back-buffer

    // draw border (NOTE: remove color pair 8 if we remove this)
    // TODO: depend on terminal size; don't hardcode lengths/alignments
    let blue = '#'.to_chtype() | pancurses::chtype::from(ColorPair(8));
    for position in grid::RECTANGLE.grow(1) {
        let Point(col, row) = position;
        term.mvaddch(row + 4, col + 30, blue);
    }

    draw_board(term, &world);

    draw_messages(term, &world.log);

    term.refresh();
}

fn draw_board(term: &Window, world: &World) {
    // TODO: depend on terminal size; don't hardcode lengths/alignments
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

fn draw_messages(term: &Window, log: &Log) {
    // TODO: depend on terminal size; don't hardcode lengths/alignments;
    //       visually group messages by turn (underscore separators? color?);
    //       maybe group similar messages; add linewrapping if needed
    for (i, &(_, ref message)) in log.recent_messages().iter().take(6).enumerate() {
        term.mvaddstr(23 - i as i32, 0, &message);
    }
}
