use const_format;
use phf;

// These constants should not be changed because it would affect the board visuals
// The board visuals are hard coded and NOT dynamically created,
// so it is difficult for these constants to be changed.
// These constants are only for being easily called multiple times for other uses
pub const BOARD_LENGTH_BY_TILES: usize = 11;
pub const BOARD_TOTAL_NUMBER_OF_TILES: usize = BOARD_LENGTH_BY_TILES * 4 - 4;
pub const TILE_LENGTH_BY_CHAR: usize = 7; // See Board::display_board() for use

#[rustfmt::skip]
pub static DISPLAY_BOARD_COORDS: [[i16; 2]; BOARD_TOTAL_NUMBER_OF_TILES] = [
    // Coordinates are based on board tiles being 7 characters in length and 3 characters tall
    // The sides (excluding the corner) are grouped starting from the bottom going clockwise
    // The coordinates (col, row) are for where to place cursors to update the tile
    // The tiles are in the same order as the board: starting from GO and going clockwise
    // Ex: To update info on Boardwalk (last tile before GO and last element in Board::board[]),
    // move the cursor to the coordinates described in the last element of this array
    [82, 33], // GO tile
    [74, 33], [66, 33], [58, 33], [50, 33], [42, 33], [34, 33], [26, 33], [18, 33], [10, 33],
    [2, 33], // Visiting Jail
    [2, 30], [2, 27], [2, 24], [2, 21], [2, 18], [2, 15], [2, 12], [2, 9], [2, 6],
    [2, 3], // Free Parking
    [10, 3], [18, 3], [26, 3], [34, 3], [42, 3], [50, 3], [58, 3], [66, 3], [74, 3],
    [82, 3], // Go to Jail
    [82, 6], [82, 9], [82, 12], [82, 15], [82, 18], [82, 21], [82, 24], [82, 27], [82, 30],
];

// Colour the background of ▔ so it looks like the tile has a colour set
// but is still contained within the tile (by the ▔ character)
// NOTE: we do not colour the foreground; different terminals may use various
// foreground colours. We assume white (or similar) foreground for contrast.
// This may be a problem in the future.
const COLOUR_TEXT: &'static str = const_format::str_repeat!("▔", TILE_LENGTH_BY_CHAR);
const END_COLOR: &'static str = "\x1b[0m";

// As the logic iterates through all the tiles and colours it based on that tile's set,
// some of those tile's set may not be defined in the association below (no colour).
// We are not going to define the map for every possible set name for scalability,
// so we are going to use this default value: ▔ (top border char) with no colour
pub const DEFAULT_COLOUR_STRING: &'static str = const_format::str_repeat!("▔", TILE_LENGTH_BY_CHAR);

// We use our own str instead of a external crate (e.g. ansi_term). Those usually uses
// a variety of structs and cannot be used inside static functions. By creating our own
// barebones ANSI background colour codes, we can create this string lookup table at compile time
// As mentioned in above NOTE, this may be a problem if we need to ensure white foreground colour
// Currently, only colours for Streets (denoted by their set names), Railroad, and Utility
pub static SET_NAME_TO_COLOUR_STRING: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "Red" => const_format::concatcp!("\x1b[41m", COLOUR_TEXT, END_COLOR),
    "Orange" => const_format::concatcp!("\x1b[48;5;166m", COLOUR_TEXT, END_COLOR),
    "Yellow" => const_format::concatcp!("\x1b[43m", COLOUR_TEXT, END_COLOR),
    "Green" => const_format::concatcp!("\x1b[42m", COLOUR_TEXT, END_COLOR),
    "Cyan" => const_format::concatcp!("\x1b[46m", COLOUR_TEXT, END_COLOR),
    "Blue" => const_format::concatcp!("\x1b[44m", COLOUR_TEXT, END_COLOR),
    "Magenta" => const_format::concatcp!("\x1b[45m", COLOUR_TEXT, END_COLOR),
    "Brown" => const_format::concatcp!("\x1b[48;5;94m", COLOUR_TEXT, END_COLOR),
    "Railroad" => const_format::concatcp!("\x1b[100m",  COLOUR_TEXT, END_COLOR), // Gray
    "Utility" => const_format::concatcp!("\x1b[47m",  COLOUR_TEXT, END_COLOR), // Whit
};
