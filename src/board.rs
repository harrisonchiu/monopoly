#![allow(dead_code)] // leave this here until done testing
#![allow(unused_variables)]

use std::collections::{HashMap, HashSet};
type TileSetContainer = HashMap<String, HashSet<String>>;

use board_tile;
use constants;

pub struct Board {
    board: Vec<board_tile::BoardTile>,
    sets: TileSetContainer,
}

impl Board {
    pub fn new(board: Vec<board_tile::BoardTile>) -> Self {
        let tile_sets: TileSetContainer = Self::organize_sets(&board);
        Self {
            board: board,
            sets: tile_sets,
        }
    }

    fn organize_sets(board: &Vec<board_tile::BoardTile>) -> TileSetContainer {
        // Create map to collect all the tiles of the same set into one set
        // Ex: { "Blue": {"Park Place", "BoardWalk"} } or { "Chance": {"Chance1", ...} }
        // This collection makes it easy to look for other tiles of the same set/type
        let mut tile_sets: TileSetContainer = TileSetContainer::new();
        for tile in board {
            let tile_set_name: String;
            match tile {
                board_tile::BoardTile::StreetTile(property) => {
                    tile_set_name = property.get_tile_name();
                }
                board_tile::BoardTile::RailroadTile(property) => {
                    tile_set_name = property.get_tile_name();
                }
                board_tile::BoardTile::UtilityTile(property) => {
                    tile_set_name = property.get_tile_name();
                }
                board_tile::BoardTile::EventTile(event) => {
                    tile_set_name = event.get_tile_name();
                }
            }

            match tile_sets.contains_key(&tile_set_name) {
                true => {
                    tile_sets
                        .get_mut(&tile_set_name)
                        .unwrap()
                        .insert(tile_set_name.clone());
                }
                false => {
                    tile_sets.insert(
                        tile_set_name.clone(),
                        HashSet::<String>::from([tile_set_name.clone()]),
                    );
                }
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
        println!("|       | VIRGINIA AVE                                                  CHANCE? |       |");
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
        self.display_board_tiles_colour();
        self.display_board_tiles_info();
    }

    fn display_board_tiles_colour(&self) {
        for (i, tile) in self.board.iter().enumerate() {
            self.reset_cursor_to_start();
            print!(
                "\x1B[{1};{0}H", // {line_number};{character_col} in the terminal
                constants::DISPLAY_BOARD_COORDS[i][0],
                constants::DISPLAY_BOARD_COORDS[i][1]
            );
            match tile {
                board_tile::BoardTile::StreetTile(property) => {
                    print!("{}", property.get_set_colour_string());
                }
                board_tile::BoardTile::RailroadTile(property) => {
                    print!("{}", property.get_set_colour_string());
                }
                board_tile::BoardTile::UtilityTile(property) => {
                    print!("{}", property.get_set_colour_string());
                }
                board_tile::BoardTile::EventTile(event) => {
                    print!("{}", event.get_set_colour_string());
                }
            }
        }
        print!("\x1B[40;1H");
    }

    fn display_board_tiles_info(&self) {
        for (i, tile) in self.board.iter().enumerate() {
            self.reset_cursor_to_start();
            print!(
                "\x1B[{1};{0}H", // {line_number};{character_col} in the terminal
                constants::DISPLAY_BOARD_COORDS[i][0],
                constants::DISPLAY_BOARD_COORDS[i][1] + 1
            );
            match tile {
                board_tile::BoardTile::StreetTile(property) => {
                    print!(" {}", property.get_property_information_string());
                }
                board_tile::BoardTile::RailroadTile(property) => {
                    print!(" {}", property.get_property_information_string());
                }
                board_tile::BoardTile::UtilityTile(property) => {
                    print!(" {}", property.get_property_information_string());
                }
                board_tile::BoardTile::EventTile(event) => {
                    // print!("{}", "|       |");
                    // print!("{}", event.get_property_information_string());
                }
            }
        }
        print!("\x1B[40;1H");
    }
}
