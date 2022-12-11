use display;
use tiles::board_tile::BoardTile;

use const_format;

// These constants are only to easily refer to dimensions of the illustrated board
// The board visuals are hard coded and NOT dynamically created and many things depend
// on its fixed visual nature (e.g. placements of each player on a tile), so it is
// difficult for these constants to be changed.
pub const BOARD_LENGTH_BY_TILES: usize = 11;
pub const BOARD_TOTAL_NUMBER_OF_TILES: usize = BOARD_LENGTH_BY_TILES * 4 - 4;
pub const BOARD_HEIGHT_BY_CHAR: usize = 38;
pub const TILE_LENGTH_BY_CHAR: usize = 7;

// Uncoloured string of the tile's top border as seen in board::display_board().
// This is used in tile structs to colour the top border.
pub const TILE_COLOURED_REGION: &'static str = const_format::str_repeat!("▔", TILE_LENGTH_BY_CHAR);

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

pub struct Board {
    board: Vec<BoardTile>,
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

        Self { board: board }
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
        self.display_all_tile_colours();
        self.display_all_property_information();
    }

    fn display_all_tile_colours(&self) {
        for tile in &self.board {
            match tile {
                BoardTile::Street(property) => property.display_tile_colour(),
                BoardTile::Railroad(property) => property.display_tile_colour(),
                BoardTile::Utility(property) => property.display_tile_colour(),
                BoardTile::Event(_) => (), // EventTiles do not display their colour set
            }
        }
    }

    fn display_all_property_information(&self) {
        for tile in &self.board {
            match tile {
                BoardTile::Street(property) => property.display_property_information(),
                BoardTile::Railroad(property) => property.display_property_information(),
                BoardTile::Utility(property) => property.display_property_information(),
                BoardTile::Event(_) => (), // EventTiles has no ownership info to display
            }
        }
    }

    pub fn get_tile(&self, position: usize) -> &BoardTile {
        self.board.get(position).unwrap()
    }

    pub fn get_tile_mut(&mut self, position: usize) -> &mut BoardTile {
        self.board.get_mut(position).unwrap()
    }

    pub fn get_set_name_from_position(&self, position: usize) -> &String {
        match self.get_tile(position) {
            BoardTile::Street(property) => &property.set_name,
            BoardTile::Railroad(property) => &property.set_name,
            BoardTile::Utility(property) => &property.set_name,
            BoardTile::Event(tile) => &tile.set_name,
        }
    }

    pub fn get_tile_name(&self, position: usize) -> &String {
        match self.get_tile(position) {
            BoardTile::Street(property) => &property.name,
            BoardTile::Railroad(property) => &property.name,
            BoardTile::Utility(property) => &property.name,
            BoardTile::Event(tile) => &tile.name,
        }
    }
}
