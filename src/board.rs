use std::collections::{HashMap, HashSet};
type TileSetContainer = HashMap<String, HashSet<String>>;

use constants;
use display;
use tiles::board_tile::BoardTile;

pub struct Board {
    board: Vec<BoardTile>,
    sets: TileSetContainer,
}

impl Board {
    pub fn new(board: Vec<BoardTile>) -> Self {
        if board.len() != constants::BOARD_TOTAL_NUMBER_OF_TILES {
            // Many things are dependent on the board being 40 tiles such as
            // the board display and how the player stores its location on the board
            panic!(
                "The board MUST have {number_of_tiles} tiles! It could be of any tiles, \
                but it must be {number_of_tiles}. Either the JSON board tile definitions do not \
                define {number_of_tiles} tiles or something went wrong when parsing the JSON.",
                number_of_tiles = constants::BOARD_TOTAL_NUMBER_OF_TILES
            );
        }

        Self {
            sets: Self::organize_sets(&board),
            board: board,
        }
    }

    fn organize_sets(board: &Vec<BoardTile>) -> TileSetContainer {
        // Create map to collect all the tiles of the same set into one set
        // Ex: { "Blue": {"Park Place", "BoardWalk"} } or { "Chance": {"Chance1", ...} }
        // This collection makes it easy to look for other tiles of the same set/type
        let mut tile_sets: TileSetContainer = TileSetContainer::new();
        for tile in board {
            // Insert tile name into the set if the group exists or create a new set of that group
            tile_sets
                .entry(tile.get_set_name().to_string())
                .or_insert(HashSet::from([tile.get_tile_name()]))
                .insert(tile.get_tile_name());
        }
        return tile_sets;
    }

    pub fn get_tile(&mut self, position: usize) -> &mut BoardTile {
        // Use the returned BoardTile to modify it (purchase, trade, build, etc.)
        // Assume position is in bounds and thus, no error in getting the BoardTile
        self.board
            .get_mut(position)
            .expect("Could not get tile at the given position; position out of bounds")
    }

    pub fn display_board(&self) {
        // Fixed and unchanging board in terms of tile names, tile locations, and dimensions
        // Please do NOT change the width and length dimensions of the board or individual tiles
        // Each tile has a width of <TILE_SIZE_BY_CHAR> characters. Update it accordingly.
        display::clear_display();
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
        println!("|       | PENN RR                                                      SHORT RR |       |");
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
            print!(
                "\x1B[{1};{0}H", // {line_number};{character_col} in the terminal
                constants::DISPLAY_BOARD_COORDS[i][0],
                constants::DISPLAY_BOARD_COORDS[i][1]
            );
            print!("{}", tile.get_set_colour_string());
        }
    }

    fn display_board_tiles_info(&self) {
        for (i, tile) in self.board.iter().enumerate() {
            print!(
                "\x1B[{1};{0}H", // {line_number};{character_col} in the terminal
                constants::DISPLAY_BOARD_COORDS[i][0],
                constants::DISPLAY_BOARD_COORDS[i][1] + 1 // 2nd row of tile
            );
            match tile {
                BoardTile::Street(property) => {
                    print!(" {}", property.get_property_information_string());
                }
                BoardTile::Railroad(property) => {
                    print!(" {}", property.get_property_information_string());
                }
                BoardTile::Utility(property) => {
                    print!(" {}", property.get_property_information_string());
                }
                BoardTile::Event(event) => {
                    // No need to display info for event tiles
                }
            }
        }
    }
}
