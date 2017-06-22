use dungeon::{self, Dungeon};
use item;
use player::Player;
use std;
use std::io::Write;
use log::Log;


// represents the game world; gets passed around everywhere
pub struct World {
    pub dungeon: Dungeon,
    pub player: Player,
    pub item_appearance_map: item::AppearanceMap,

    pub player_appearance_byte: u8,
    pub door_appearance_byte: u8,
    pub wall_appearance_byte: u8,

    pub log: Log
}

impl World {
    pub fn new() -> World {
        let mut dungeon = dungeon::build().unwrap_or_else(|e| {
            writeln!(std::io::stderr(), "{}", e).unwrap();
            std::process::exit(1)
        });

        let mut log = Log::new();

        World {
            player: Player::new(&mut log, &mut dungeon),
            dungeon: dungeon,
            item_appearance_map: item::random_appearance_map(),

            player_appearance_byte: 0b11111111, // white @
            door_appearance_byte: 0b11001010, // brown +
            wall_appearance_byte: 0b01100010, // teal #

            log: log
        }
    }
}
