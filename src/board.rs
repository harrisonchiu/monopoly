use std::collections::{HashMap, HashSet};
type TileSetContainer = HashMap<String, HashSet<String>>;

use display;
use tiles::board_tile::BoardTile;

use const_format;
use phf;

// These constants are only to easily refer to dimensions of the illustrated board
// The board visuals are hard coded and NOT dynamically created and many things depend
// on its fixed visual nature (e.g. placements of each player on a tile), so it is
// difficult for these constants to be changed.
pub const BOARD_LENGTH_BY_TILES: usize = 11;
pub const BOARD_TOTAL_NUMBER_OF_TILES: usize = BOARD_LENGTH_BY_TILES * 4 - 4;
pub const BOARD_HEIGHT_BY_CHAR: usize = 38;
pub const TILE_LENGTH_BY_CHAR: usize = 7;

#[rustfmt::skip]
pub static DISPLAY_BOARD_COORDS: [[u8; 2]; BOARD_TOTAL_NUMBER_OF_TILES] = [
    // Coordinates for each tile on the board based on each tile being 7 char wide and 3 char tall
    // The coordinates refer to top left char that is within the tile (the char after "|")
    // The tiles are in the same order as how the tiles are inserted into the Board.board::Vec<>
    // e.g. Board walk is the last element and GO is the first element
    [82, 33], // GO tile
    [74, 33], [66, 33], [58, 33], [50, 33], [42, 33], [34, 33], [26, 33], [18, 33], [10, 33],
    [2, 33], // Visiting Jail
    [2, 30], [2, 27], [2, 24], [2, 21], [2, 18], [2, 15], [2, 12], [2, 9], [2, 6],
    [2, 3], // Free Parking
    [10, 3], [18, 3], [26, 3], [34, 3], [42, 3], [50, 3], [58, 3], [66, 3], [74, 3],
    [82, 3], // Go to Jail
    [82, 6], [82, 9], [82, 12], [82, 15], [82, 18], [82, 21], [82, 24], [82, 27], [82, 30],
];

// Some tiles may not have colours associated to their sets. They will be uncoloured.
pub const UNCOLOURED_REGION: &'static str = const_format::str_repeat!("▔", TILE_LENGTH_BY_CHAR);

// No need for external crates for a simple coloured section, this is sufficient
// Only coloured the background, so text foreground may be an issue. We assume white foreground
const COLOUR_TEXT: &'static str = const_format::str_repeat!("▔", TILE_LENGTH_BY_CHAR);
const END_COLOUR: &'static str = "\x1b[0m";
pub static COLOURED_REGION_OF_EACH_SET: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "Red" => const_format::concatcp!("\x1b[41m", COLOUR_TEXT, END_COLOUR),
    "Orange" => const_format::concatcp!("\x1b[48;5;166m", COLOUR_TEXT, END_COLOUR),
    "Yellow" => const_format::concatcp!("\x1b[43m", COLOUR_TEXT, END_COLOUR),
    "Green" => const_format::concatcp!("\x1b[42m", COLOUR_TEXT, END_COLOUR),
    "Cyan" => const_format::concatcp!("\x1b[46m", COLOUR_TEXT, END_COLOUR),
    "Blue" => const_format::concatcp!("\x1b[44m", COLOUR_TEXT, END_COLOUR),
    "Magenta" => const_format::concatcp!("\x1b[45m", COLOUR_TEXT, END_COLOUR),
    "Brown" => const_format::concatcp!("\x1b[48;5;94m", COLOUR_TEXT, END_COLOUR),
    "Railroad" => const_format::concatcp!("\x1b[100m",  COLOUR_TEXT, END_COLOUR), // Gray
    "Utility" => const_format::concatcp!("\x1b[47m",  COLOUR_TEXT, END_COLOUR), // White
};

pub struct Board {
    board: Vec<BoardTile>,
    sets: TileSetContainer,
}

impl Board {
    pub fn new(board: Vec<BoardTile>) -> Self {
        if board.len() != BOARD_TOTAL_NUMBER_OF_TILES {
            // Many things are dependent on the board being 40 tiles such as
            // the board display and how the player stores its location on the board
            panic!(
                "The board MUST have {number_of_tiles} tiles! It could be of any tiles, \
                but it must be {number_of_tiles}. Either the JSON board tile definitions do not \
                define {number_of_tiles} tiles or something went wrong when parsing the JSON.",
                number_of_tiles = BOARD_TOTAL_NUMBER_OF_TILES
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

    pub fn get_tile(&mut self, position: usize) -> &BoardTile {
        // Use the returned BoardTile to modify it (purchase, trade, build, etc.)
        // Assume position is in bounds and thus, no error in getting the BoardTile
        self.board
            .get(position)
            .expect("Could not get tile at the given position; position out of bounds")
    }

    pub fn get_tile_mut(&mut self, position: usize) -> &mut BoardTile {
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
                DISPLAY_BOARD_COORDS[i][0], DISPLAY_BOARD_COORDS[i][1]
            );
            print!("{}", tile.get_set_colour_string());
        }
    }

    fn display_board_tiles_info(&self) {
        for (i, tile) in self.board.iter().enumerate() {
            print!(
                "\x1B[{1};{0}H", // {line_number};{character_col} in the terminal
                DISPLAY_BOARD_COORDS[i][0],
                DISPLAY_BOARD_COORDS[i][1] + 1 // 2nd row of tile
            );
            match tile {
                BoardTile::Street(property) => {
                    print!("{}", property.get_property_information_string());
                }
                BoardTile::Railroad(property) => {
                    print!("{}", property.get_property_information_string());
                }
                BoardTile::Utility(property) => {
                    print!("{}", property.get_property_information_string());
                }
                BoardTile::Event(_) => {
                    // No need to display info for event tiles
                }
            }
        }
    }
}
