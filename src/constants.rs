// These constants should not be changed because it would affect the board visuals
// The board visuals are hard coded and NOT dynamically created,
// so it is difficult for these constants to be changed.
// These constants are only for being easily called multiple times for other uses
pub const BOARD_LENGTH_BY_TILES: usize = 11;
pub const BOARD_TOTAL_NUMBER_OF_TILES: usize = BOARD_LENGTH_BY_TILES * 4 - 4;
pub const TILE_LENGTH_BY_CHAR: usize = 7; // See Board::display_board() for use
pub type BoardCoordsArray = [[i16; 2]; BOARD_TOTAL_NUMBER_OF_TILES];

#[rustfmt::skip]
pub static DISPLAY_BOARD_COORDS: BoardCoordsArray = [
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
