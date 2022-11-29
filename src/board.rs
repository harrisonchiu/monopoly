#![allow(dead_code)] // leave this here until done testing
#![allow(unused_variables)]

use std::collections::{HashMap, HashSet};
type TileSet = HashMap<String, HashSet<String>>;

use serde_json;

use constants;

#[allow(dead_code)]
pub struct Board {
    board: [BoardTile; 1],
    pub board_graphical_coords: [[usize; 2]; constants::BOARD_TOTAL_NUMBER_OF_TILES],
    sets: TileSet,
}

impl Board {
    pub fn new(board: [BoardTile; 1]) -> Self {
        let tile_sets: TileSet = Self::organize_sets(&board);
        Self {
            board: board,
            board_graphical_coords: constants::BOARD_DISPLAY_COORDS,
            sets: tile_sets,
        }
    }

    fn organize_sets(board: &[BoardTile; 1]) -> TileSet {
        // Create map to collect all the tiles of the same set into one set
        // Ex: { "Blue": {"Park Place", "BoardWalk"} } or { "Chance": {"Chance1", ...} }
        // This collection makes it easy to look for other tiles of the same set/type
        let mut tile_sets = TileSet::new();
        for tile in board {
            let tile_set_name = tile.get_set_name();
            if tile_sets.contains_key(&tile_set_name) {
                tile_sets
                    .get_mut(&tile_set_name)
                    .unwrap()
                    .insert(tile.get_tile_name());
            } else {
                tile_sets.insert(
                    tile_set_name,
                    HashSet::<String>::from([tile.get_tile_name()]),
                );
            }
        }

        return tile_sets;
    }

    pub fn reset_cursor_to_start(&self) {
        // Moves to the cursor to the very top left (1st row and 1st col)
        // Good default and standard location for cursor so we know to
        // always count the rows and cols from the top left
        print!("\x1B[1;1H");
    }

    pub fn clear_display(&self) {
        // Clears the screen and puts the cursor at the very beginning (top left)
        // to redraw anything from the start (1st row 1st col), otherwise it may
        // redraw from the middle where it finished clearing previous texts
        print!("\x1B[2J");
        self.reset_cursor_to_start();
    }

    pub fn display_board(&self) {
        // Fixed and unchanging board in terms of tile names, tile locations, and dimensions
        // Please do NOT change the width and length dimensions of the board or individual tiles
        // Each tile has a width of <TILE_SIZE_BY_CHAR> characters. Update it accordingly.
        self.clear_display();
        println!("  FREE   KNTUCKY           IND     ILL     B&O     ATL   VENTNOR  WATER  MARVIN   GO TO  ");
        println!("  PARK     AVE   CHANCE?   AVE     AVE     RR      AVE     AVE    WORKS  GARDENS  JAIL   ");
        println!("|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|");
        println!("|       |       |       |       |       |       |       |       |       |       |       |");
        println!("|       |       |       |       |       |       |       |       |       |       |       |");
        println!("|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|");
        println!("|       | NEW YORK AVE                                              PACIFIC AVE |       |");
        println!("|       |                                                                       |       |");
        println!("|▔▔▔▔▔▔▔|                                                                       |▔▔▔▔▔▔▔|");
        println!("|       | TENN AVE                                                 CAROLINA AVE |       |");
        println!("|       |                                                                       |       |");
        println!("|▔▔▔▔▔▔▔|                                                                       |▔▔▔▔▔▔▔|");
        println!("|       | CHEST                                                           CHEST |       |");
        println!("|       |                                                                       |       |");
        println!("|▔▔▔▔▔▔▔|                                                                       |▔▔▔▔▔▔▔|");
        println!("|       | JAMES PLACE                                                  PENN AVE |       |");
        println!("|       |                                                                       |       |");
        println!("|▔▔▔▔▔▔▔|                                                                       |▔▔▔▔▔▔▔|");
        println!("|       | PENN RR                                                 SHORT LINE RR |       |");
        println!("|       |                                                                       |       |");
        println!("|▔▔▔▔▔▔▔|                                                                       |▔▔▔▔▔▔▔|");
        println!("|1234_56| VIRGINIA AVE                                                  CHANCE? |       |");
        println!("|       |                                                                       |       |");
        println!("|▔▔▔▔▔▔▔|                                                                       |▔▔▔▔▔▔▔|");
        println!("|       | STATES AVE                                                 PARK PLACE |       |");
        println!("|       |                                                                       |       |");
        println!("|▔▔▔▔▔▔▔|                                                                       |▔▔▔▔▔▔▔|");
        println!("|       | ELEC COMPANY                                               LUXURY TAX |       |");
        println!("|       |                                                                       |       |");
        println!("|▔▔▔▔▔▔▔|                                                                       |▔▔▔▔▔▔▔|");
        println!("|       | CHARLES PLACE                                               BOARDWALK |       |");
        println!("|       |                                                                       |       |");
        println!("|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|▔▔▔▔▔▔▔|");
        println!("|       |       |       |       |       |       |       |       |       |       |       |");
        println!("|       |       |       |       |       |       |       |       |       |       |       |");
        println!(" ▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔ ");
        println!("  VISIT   CONN   VERMONT CHANCE?  ORNTL  READING INCOME  BALTIC   CHEST   MEDIT    GO    ");
        println!("  JAIL     AVE     AVE             AVE     RR      TAX     AVE             AVE           ");
    }
}

#[derive(Debug)]
pub struct BoardTile {
    tile_data: serde_json::Value,
}

pub trait PropertyTile {
    fn new(tile_data: serde_json::Value) -> Self;
    fn get_tile_name(&self) -> String;
    fn get_set_name(&self) -> String;
}

impl PropertyTile for BoardTile {
    fn new(tile_data: serde_json::Value) -> Self {
        Self {
            tile_data: tile_data,
        }
    }

    fn get_tile_name(&self) -> String {
        return self.tile_data["name"].to_string();
    }

    fn get_set_name(&self) -> String {
        return self.tile_data["set"].to_string();
    }
}
