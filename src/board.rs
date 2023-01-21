use itertools::{Interleave, Itertools};
use std::{iter::Iterator, ops::RangeBounds};

use board_tiles::tile::Tile;
use std::str::SplitWhitespace;

pub struct BoardDisplay {
    pub ascii_board: String,
    pub ascii_board_size: (usize, usize), // (width, height)
    visual_tile_order: Vec<usize>,
    actual_tile_order: Vec<usize>,
    colour_indices: Vec<usize>,
    detail_indices: Vec<usize>,
    player_indices: Vec<usize>,
    pub player_coords: Vec<(usize, usize)>,
}

impl BoardDisplay {
    const NUMBER_OF_TILES: usize = 40;
    const TILE_WIDTH: usize = 7;

    // These placeholders must have the same length as the string that will replace it
    // Be careful with unicode chars as they decompose into multiple chars, subtly increasing length
    // Ex: placeholder="1234567" replacer="\x1b[XXm567" => chars: {\x1b, [, X, X, m, 5, 6, 7}
    // All replacers must have the same length
    const TILE_COLOUR_PLACEHOLDER: &str = "<<COLOURS>>";
    const TILE_DETAIL_PLACEHOLDER: &str = "$1234 \x1b[00000049mX\x1b[0m";

    // We will probably not use this because that means we mutate the string every turn
    // Probably very inefficient. Also players aren't inherent to the board, we can just print ontop
    // CHOSEN NOT TO DRAW PLAYERS INTO THE BOARD STRING: REQUIRES REFRESHING ENTIRE BOARD EVERYTURN
    // CAUSES THE BOARD TO FLICKER EVERY TURN. FLICKER FOR TILE CHANGES IS ENOUGH
    const TILE_PLAYER_PLACEHOLDER: &str = " _ _ _ ";
    // Must be <TILE_WIDTH> long (i.e. TILE_WIDTH ASCII chars) if using print() onto terminal method
    // Otherwise, use the below commented str as the TILE_PLAYER_PLACEHOLDER
    // "\x1b[49mA\x1b[0m \x1b[49mB\x1b[0m \x1b[49mC\x1b[0m \x1b[49mD\x1b[0m";

    pub fn new(board: &Vec<Tile>) -> Self {
        let ascii_board: String = Self::get_ascii_board(&board);

        let mut board_display = Self {
            visual_tile_order: Self::get_visual_tiles_order(),
            actual_tile_order: Self::get_actual_tiles_order(),
            colour_indices: find_substring_matches(&ascii_board, Self::TILE_COLOUR_PLACEHOLDER),
            detail_indices: find_substring_matches(&ascii_board, Self::TILE_DETAIL_PLACEHOLDER),
            player_indices: find_substring_matches(&ascii_board, Self::TILE_PLAYER_PLACEHOLDER),
            player_coords: Self::get_player_row_coords(
                &ascii_board,
                &mut Self::get_visual_tiles_order(), // Needs its own; it consumes it
            ),
            ascii_board_size: (
                ascii_board.lines().next().unwrap_or_default().len(),
                ascii_board.lines().count(),
            ),
            ascii_board,
        };

        board_display.update_tile_colours(&board, 0..40);
        board_display.update_tile_details(&board, 0..40);
        // board_display.update_tile_players(&board, 0..40);

        board_display
    }

    fn get_visual_tiles_order() -> Vec<usize> {
        //! Given the actual tile ids (index), the visual tile ids are returned (value)
        //! Actual tile ids are the unique labeling of each tile starting from GO
        //! increasing by 1 where ever the next tile to go to is, ending at the last tile
        //! Visual tile ids are the unique labeling of each tile starting from the first tile that
        //! is iterated when iterating through the board string (top to bottom, left to right)
        ((29..40).rev())
            .chain((11..28).step_by(2).rev())
            .chain(0..=10)
            .chain((12..29).step_by(2))
            .collect()
    }

    fn get_actual_tiles_order() -> Vec<usize> {
        //! Given the visual tile ids (index), the actual tile ids are returned (value)
        //! Actual tile ids are the unique labeling of each tile starting from GO
        //! increasing by 1 where ever the next tile to go to is, ending at the last tile
        //! Visual tile ids are the unique labeling of each tile starting from the first tile that
        //! is iterated when iterating through the board string (top to bottom, left to right)
        ((20..=30).rev())
            .chain((11..=19).rev().interleave(31..=39))
            .chain((0..=10).rev())
            .collect()
    }

    fn get_player_row_coords(board: &str, actual_tile_order: &mut [usize]) -> Vec<(usize, usize)> {
        //! Creates a vec of coorindates for the start of every player row (where players are
        //! drawn on the tiles) in the order of the actual tile order. This lets us iterate
        //! through the coordinate vector and print to the terminal at the exect coordinates
        let mut user_row_positions: Vec<(usize, usize)> = Vec::with_capacity(Self::NUMBER_OF_TILES);
        for (line_number, line) in board.lines().enumerate() {
            let tiles: Vec<usize> = find_substring_matches(line, Self::TILE_PLAYER_PLACEHOLDER);
            for tile_index in tiles {
                user_row_positions.push((line_number, tile_index + 1));
            }
        }

        // Right now, these indices are in visual tile order, we want them in actual tile order
        // so we can draw player positions. Player positions are done through actual tile order

        sort_by_indices(&mut user_row_positions, actual_tile_order);
        user_row_positions
    }

    fn update_tile_colours<R>(&mut self, board: &[Tile], tiles: R)
    where
        R: RangeBounds<usize> + Iterator<Item = usize>,
    {
        //! We cannot iterate each char of the ascii_board string because the board borders
        //! and the colour codes are non-ASCII chars, so they decompose into: [\u{XXX}, YY]
        //! Thus, we first find all the indices that we may want to change <self.colour_indices>
        //! Assume the PLACEHOLDERS have the same length as the replacer INCLUDING THE UNICODES
        //! @tiles: Range or InclusiveRange for which tile(s) to add colour to (OVERWRITES IT)
        //! @return: None, inplace string mutation

        for actual_tile_id in tiles {
            if actual_tile_id >= self.visual_tile_order.len() {
                break;
            }

            let colour_index = self.colour_indices[self.visual_tile_order[actual_tile_id]];
            self.ascii_board.replace_range(
                colour_index..(colour_index + Self::TILE_COLOUR_PLACEHOLDER.len()),
                &board[actual_tile_id].get_colour(),
            );
        }
    }

    pub fn update_tile_details<R>(&mut self, board: &[Tile], tiles: R)
    where
        R: RangeBounds<usize> + Iterator<Item = usize>,
    {
        //! This is a tile row that can change throughout the game
        for actual_tile_id in tiles {
            if actual_tile_id >= self.visual_tile_order.len() {
                break;
            }

            let detail_index = self.detail_indices[self.visual_tile_order[actual_tile_id]];
            self.ascii_board.replace_range(
                detail_index..(detail_index + Self::TILE_DETAIL_PLACEHOLDER.len()),
                &board[actual_tile_id].get_details_row(),
            );
        }
    }

    fn get_ascii_board(board: &[Tile]) -> String {
        const T: usize = 7; // Width of a tile excluding | chars; 1 char var name to prettify board
        const NAMELEN: usize = T * 2; // 7 char var name to make side tiles line up, prettify board
        const LEFT_INDENT: &str = "    ";

        // Because of how the board and each tile name is visualized, the arguments to formatted
        // string cannot be just a vector of the 40 names. We must break up some names into words.
        let mut board_tile_names: Vec<String> = Vec::with_capacity(Self::NUMBER_OF_TILES);
        for (tile_id, tile) in board.iter().enumerate() {
            if (0..=10).contains(&tile_id) {
                let mut words: SplitWhitespace = tile.get_display_name().split_whitespace();
                board_tile_names.push(words.next().unwrap_or_default().to_string());
                board_tile_names.push(words.next().unwrap_or_default().to_string());
            } else if (20..=30).contains(&tile_id) {
                let mut words: SplitWhitespace = tile.get_display_name().split_whitespace();
                board_tile_names.push(words.next().unwrap_or_default().to_string());
                board_tile_names.push(words.next().unwrap_or_default().to_string());
            } else {
                board_tile_names.push(tile.get_display_name().to_owned());
            }
        }

        // The board as a string with the tile's names as the parameters
        let ascii_board: String = {
            // Curly braces are to prevent rustfmt from formatting the parameters inside this block
            format!("
{LEFT_INDENT} {31:^7} {33:^7} {35:^7} {37:^7} {39:^7} {41:^7} {43:^7} {45:^7} {47:^7} {49:^7} {51:^7} 
{LEFT_INDENT} {32:^7} {34:^7} {36:^7} {38:^7} {40:^7} {42:^7} {44:^7} {46:^7} {48:^7} {50:^7} {52:^7} 
{LEFT_INDENT}|{C}{BX}|{C}{BX}|{C}{BX}|{C}{BX}|{C}{BX}|{C}{BX}|{C}{BX}|{C}{BX}|{C}{BX}|{C}{BX}|{C}{BX}|
{LEFT_INDENT}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|
{LEFT_INDENT}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|
{LEFT_INDENT}|{C}{BX}|▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔|{C}{BX}|
{LEFT_INDENT}|{INFOS}| {30:<NAMELEN$}  |                                   |  {53:>NAMELEN$} |{INFOS}|
{LEFT_INDENT}|{USERS}|                 |                                   |                 |{USERS}|
{LEFT_INDENT}|{C}{BX}|                 |                                   |                 |{C}{BX}|
{LEFT_INDENT}|{INFOS}| {29:<NAMELEN$}  |                                   |  {54:>NAMELEN$} |{INFOS}|
{LEFT_INDENT}|{USERS}|                 |                                   |                 |{USERS}|
{LEFT_INDENT}|{C}{BX}|                 |                                   |                 |{C}{BX}|
{LEFT_INDENT}|{INFOS}| {28:<NAMELEN$}  |                                   |  {55:>NAMELEN$} |{INFOS}|
{LEFT_INDENT}|{USERS}|                 |                                   |                 |{USERS}|
{LEFT_INDENT}|{C}{BX}|                 |                                   |                 |{C}{BX}|
{LEFT_INDENT}|{INFOS}| {27:<NAMELEN$}  |                                   |  {56:>NAMELEN$} |{INFOS}|
{LEFT_INDENT}|{USERS}|                 |                                   |                 |{USERS}|
{LEFT_INDENT}|{C}{BX}|                 |                                   |                 |{C}{BX}|
{LEFT_INDENT}|{INFOS}| {26:<NAMELEN$}  |                                   |  {57:>NAMELEN$} |{INFOS}|
{LEFT_INDENT}|{USERS}|                 |                                   |                 |{USERS}|
{LEFT_INDENT}|{C}{BX}|                 |                                   |                 |{C}{BX}|
{LEFT_INDENT}|{INFOS}| {25:<NAMELEN$}  |                                   |  {58:>NAMELEN$} |{INFOS}|
{LEFT_INDENT}|{USERS}|                 |                                   |                 |{USERS}|
{LEFT_INDENT}|{C}{BX}|                 |                                   |                 |{C}{BX}|
{LEFT_INDENT}|{INFOS}| {24:<NAMELEN$}  |                                   |  {59:>NAMELEN$} |{INFOS}|
{LEFT_INDENT}|{USERS}|                 |                                   |                 |{USERS}|
{LEFT_INDENT}|{C}{BX}|                 |                                   |                 |{C}{BX}|
{LEFT_INDENT}|{INFOS}| {23:<NAMELEN$}  |                                   |  {60:>NAMELEN$} |{INFOS}|
{LEFT_INDENT}|{USERS}|                 |                                   |                 |{USERS}|
{LEFT_INDENT}|{C}{BX}|                 |                                   |                 |{C}{BX}|
{LEFT_INDENT}|{INFOS}| {22:<NAMELEN$}  |                                   |  {61:>NAMELEN$} |{INFOS}|
{LEFT_INDENT}|{USERS}|                                                                       |{USERS}|
{LEFT_INDENT}|{C}{BX}|{C}{BX}|{C}{BX}|{C}{BX}|{C}{BX}|{C}{BX}|{C}{BX}|{C}{BX}|{C}{BX}|{C}{BX}|{C}{BX}|
{LEFT_INDENT}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|{INFOS}|
{LEFT_INDENT}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|{USERS}|
{LEFT_INDENT} ▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔ 
{LEFT_INDENT} {20:^7} {18:^7} {16:^7} {14:^7} {12:^7} {10:^7} {08:^7} {06:^7} {04:^7} {02:^7} {00:^7} 
{LEFT_INDENT} {21:^7} {19:^7} {17:^7} {15:^7} {13:^7} {11:^7} {09:^7} {07:^7} {05:^7} {03:^7} {01:^7}\n",
    // Spaces between each name for the top and bottom rows are necessary because we count the width
    // of the tile as the # of space between the | chars, thus excluding them and creating a gap

    // The groups of * chars are to denote the tile's row that will draw its colour
    // The groups of P chars are to denote the tile's row that will show its price and property info
    // These groups MUST be unique for functions to find the substrings and replace them with
    // the actual requested information. This means NO tile names can equal them!

    // Bottom tiles
    board_tile_names[00], board_tile_names[01], board_tile_names[02], board_tile_names[03],
    board_tile_names[04], board_tile_names[05], board_tile_names[06], board_tile_names[07],
    board_tile_names[08], board_tile_names[09], board_tile_names[10], board_tile_names[11],
    board_tile_names[12], board_tile_names[13], board_tile_names[14], board_tile_names[15],
    board_tile_names[16], board_tile_names[17], board_tile_names[18], board_tile_names[19],
    board_tile_names[20], board_tile_names[21],

    // Left tiles
    board_tile_names[22], board_tile_names[23], board_tile_names[24], board_tile_names[25],
    board_tile_names[26], board_tile_names[27], board_tile_names[28], board_tile_names[29],
    board_tile_names[30],

    // Top tiles
    board_tile_names[31], board_tile_names[32], board_tile_names[33], board_tile_names[34],
    board_tile_names[35], board_tile_names[36], board_tile_names[37], board_tile_names[38],
    board_tile_names[39], board_tile_names[40], board_tile_names[41], board_tile_names[42],
    board_tile_names[43], board_tile_names[44], board_tile_names[45], board_tile_names[46],
    board_tile_names[47], board_tile_names[48], board_tile_names[49], board_tile_names[50],
    board_tile_names[51], board_tile_names[52],

    // Right tiles
    board_tile_names[53], board_tile_names[54], board_tile_names[55], board_tile_names[56],
    board_tile_names[57], board_tile_names[58], board_tile_names[59], board_tile_names[60],
    board_tile_names[61],

    BX = "▔▔▔▔▔▔▔\x1b[0m",
    C = Self::TILE_COLOUR_PLACEHOLDER,
    INFOS = Self::TILE_DETAIL_PLACEHOLDER,
    USERS = Self::TILE_PLAYER_PLACEHOLDER,

        )
        };

        ascii_board
    }
}

fn find_substring_matches(string: &str, find: &str) -> Vec<usize> {
    //! Wrapper around match_indices(), removing the matched string, only containing the indices
    //! Useful like match_indices, but without borrowing; we can use indices vec more freely
    string.match_indices(find).map(|(index, _)| index).collect()
}

fn sort_by_indices<T>(data: &mut [T], indices: &mut [usize]) {
    //! In place O(N) sorting algorithm based on the order of a given vector
    //! Consumes and sorts @indices from custom order to ascending order!
    //! @data: the array-like to be sorted based on @indices
    //! @indices: vector's values determine the order of @data's elements. Is consumed
    //!     and must be in range [0, @indices.len), strictly increasing by 1, i.e. NO gaps
    //! Ex: fn([a, b, c, d], [0, 3, 2, 1]) -> [a, d, c, b]

    for index in 0..data.len() {
        if indices[index] == index {
            continue;
        }

        let mut current_index: usize = index;
        loop {
            let target_index: usize = indices[current_index];
            indices[current_index] = current_index;
            if indices[target_index] == target_index {
                break;
            }
            data.swap(current_index, target_index);
            current_index = target_index;
        }
    }
}
