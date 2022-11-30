#![allow(dead_code)] // leave this here until done testing
#![allow(unused_variables)]

use std::collections::{HashMap, HashSet};
type TileSetContainer = HashMap<String, HashSet<String>>;

use serde_json;

use colours;
use constants;

#[allow(dead_code)]
pub struct Board {
    board: Vec<BoardTile>,
    sets: TileSetContainer,
    display_board_coords: constants::BoardCoordsArray,
}

impl Board {
    pub fn new(board: Vec<BoardTile>) -> Self {
        let tile_sets: TileSetContainer = Self::organize_sets(&board);
        Self {
            board: board,
            sets: tile_sets,
            display_board_coords: constants::DISPLAY_BOARD_COORDS,
        }
    }

    fn organize_sets(board: &Vec<BoardTile>) -> TileSetContainer {
        // Create map to collect all the tiles of the same set into one set
        // Ex: { "Blue": {"Park Place", "BoardWalk"} } or { "Chance": {"Chance1", ...} }
        // This collection makes it easy to look for other tiles of the same set/type
        let mut tile_sets: TileSetContainer = TileSetContainer::new();
        for tile in board {
            let tile_set_name: String;
            match tile {
                BoardTile::PropertyTile(property) => {
                    tile_set_name = property.get_tile_name();
                }
                BoardTile::EventTile(event) => {
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
        self.colour_display_board_tiles();
    }

    fn colour_display_board_tiles(&self) {
        self.reset_cursor_to_start();
        for (i, tile) in self.board.iter().enumerate() {
            print!(
                "\x1B[{1};{0}H", // {line_number};{character_col} in the terminal
                self.display_board_coords[i][0], self.display_board_coords[i][1]
            );
            match tile {
                BoardTile::PropertyTile(property) => {
                    print!("{}", property.get_set_colour_string());
                }
                BoardTile::EventTile(event) => {
                    print!("{}", event.get_set_colour_string());
                }
            }
            self.reset_cursor_to_start();
        }
        print!("\x1B[40;1H");
    }
}

pub enum BoardTile {
    PropertyTile(PropertyTile),
    EventTile(EventTile),
}

pub struct PropertyTile {
    pub tile_data: serde_json::Value,
    // houses, hotel, current rent,
    // these are things that can change throughout its life
    // we can also add metadata like number of times landed on
}

pub struct EventTile {
    pub tile_data: serde_json::Value,
}

impl PropertyTile {
    pub fn new(tile_data: serde_json::Value) -> Self {
        Self {
            tile_data: tile_data,
        }
    }
    fn get_tile_name(&self) -> String {
        return self.tile_data["name"].to_string();
    }

    fn is_property_tile(&self) -> bool {
        return true;
    }

    fn is_event_tile(&self) -> bool {
        return false;
    }

    fn get_set_name(&self) -> String {
        return self.tile_data["set"].to_string();
    }

    fn get_set_colour_string(&self) -> String {
        // The top row (same row as ▔ top border) with background colour of the tile's set
        // or no background colour. It does not affect foreground colour of ▔
        return colours::SET_NAME_TO_COLOUR_STRING
            .get(&self.get_set_name().as_str())
            .unwrap_or(&colours::DEFAULT_COLOUR_STRING)
            .to_string();
    }

    // fn get_property_information_string(&self) -> String {
    //     // Within <TILE_LENGTH_BY_CHAR> length, display current cost of tile and its buildings
    // }
}

impl EventTile {
    pub fn new(tile_data: serde_json::Value) -> Self {
        Self {
            tile_data: tile_data,
        }
    }

    fn get_tile_name(&self) -> String {
        return self.tile_data["name"].to_string();
    }

    fn is_property_tile(&self) -> bool {
        return false;
    }

    fn is_event_tile(&self) -> bool {
        return true;
    }

    fn get_set_name(&self) -> String {
        return self.tile_data["set"].to_string();
    }

    fn get_set_colour_string(&self) -> String {
        // The top row (same row as ▔ top border) with background colour of the tile's set
        // or no background colour. It does not affect foreground colour of ▔
        return colours::SET_NAME_TO_COLOUR_STRING
            .get(&self.get_set_name().as_str())
            .unwrap_or(&colours::DEFAULT_COLOUR_STRING)
            .to_string();
    }
}
