use std::{str::SplitWhitespace, iter::FromIterator};

use crate::board;

pub struct Board {
    // board: Vec<i32>,
    ascii_board: String,
}

impl Board {
    pub fn new(tile_display_names: Vec<String>) -> Self {
        Self {
            ascii_board: Self::get_ascii_board(&tile_display_names),
        }
    }

    fn get_ascii_board(tile_display_names: &Vec<String>) -> String {
        const T: usize = 7; // Width of a tile excluding | chars, 1 char var name to prettify board
        const NAMELEN: usize = T * 2; // 7 char var name to make side tiles line up
        const LEFT_INDENT: &str = "    ";

        let mut board_tile_ids_from_top_left: Vec<usize> = Vec::new();
        board_tile_ids_from_top_left.extend_from_slice(&Vec::from_iter(20..=30).as_slice());
        let a = (11..=19).rev().zip(31..=39);
        board_tile_ids_from_top_left.extend_from_slice(&[1]);

        // Because of how the board and each tile name is visualized, the arguments to formatted
        // string cannot be just a vector of the 40 names. We must break up some names into words.
        let mut board_tile_names: Vec<String> = Vec::new();
        for (tile_id, name) in tile_display_names.iter().enumerate() {
            if (0..=10).contains(&tile_id) {
                let mut words: SplitWhitespace = name.split_whitespace();
                board_tile_names.push(words.next().unwrap_or_default().to_string());
                board_tile_names.push(words.next().unwrap_or_default().to_string());
            } else if (20..=30).contains(&tile_id) {
                let mut words: SplitWhitespace = name.split_whitespace();
                board_tile_names.push(words.next().unwrap_or_default().to_string());
                board_tile_names.push(words.next().unwrap_or_default().to_string());
            } else {
                board_tile_names.push(name.to_owned());
            }
        }

        #[rustfmt::skip]
        let ascii_board: String = format!("
{LEFT_INDENT} {31:^7} {33:^7} {35:^7} {37:^7} {39:^7} {41:^7} {43:^7} {45:^7} {47:^7} {49:^7} {51:^7} 
{LEFT_INDENT} {32:^7} {34:^7} {36:^7} {38:^7} {40:^7} {42:^7} {44:^7} {46:^7} {48:^7} {50:^7} {52:^7} 
{LEFT_INDENT}|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|
{LEFT_INDENT}|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|
{LEFT_INDENT}|       |       |       |       |       |       |       |       |       |       |       |
{LEFT_INDENT}|CCCCCCC|▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔|CCCCCCC|
{LEFT_INDENT}|PPPPPPP| {30:<NAMELEN$}  |                                   |  {53:>NAMELEN$} |PPPPPPP|
{LEFT_INDENT}|       |                 |                                   |                 |       |
{LEFT_INDENT}|CCCCCCC|                 |                                   |                 |CCCCCCC|
{LEFT_INDENT}|PPPPPPP| {29:<NAMELEN$}  |                                   |  {54:>NAMELEN$} |PPPPPPP|
{LEFT_INDENT}|       |                 |                                   |                 |       |
{LEFT_INDENT}|CCCCCCC|                 |                                   |                 |CCCCCCC|
{LEFT_INDENT}|PPPPPPP| {28:<NAMELEN$}  |                                   |  {55:>NAMELEN$} |PPPPPPP|
{LEFT_INDENT}|       |                 |                                   |                 |       |
{LEFT_INDENT}|CCCCCCC|                 |                                   |                 |CCCCCCC|
{LEFT_INDENT}|PPPPPPP| {27:<NAMELEN$}  |                                   |  {56:>NAMELEN$} |PPPPPPP|
{LEFT_INDENT}|       |                 |                                   |                 |       |
{LEFT_INDENT}|CCCCCCC|                 |                                   |                 |CCCCCCC|
{LEFT_INDENT}|PPPPPPP| {26:<NAMELEN$}  |                                   |  {57:>NAMELEN$} |PPPPPPP|
{LEFT_INDENT}|       |                 |                                   |                 |       |
{LEFT_INDENT}|CCCCCCC|                 |                                   |                 |CCCCCCC|
{LEFT_INDENT}|PPPPPPP| {25:<NAMELEN$}  |                                   |  {58:>NAMELEN$} |PPPPPPP|
{LEFT_INDENT}|       |                 |                                   |                 |       |
{LEFT_INDENT}|CCCCCCC|                 |                                   |                 |CCCCCCC|
{LEFT_INDENT}|PPPPPPP| {24:<NAMELEN$}  |                                   |  {59:>NAMELEN$} |PPPPPPP|
{LEFT_INDENT}|       |                 |                                   |                 |       |
{LEFT_INDENT}|CCCCCCC|                 |                                   |                 |CCCCCCC|
{LEFT_INDENT}|PPPPPPP| {23:<NAMELEN$}  |                                   |  {60:>NAMELEN$} |PPPPPPP|
{LEFT_INDENT}|       |                 |                                   |                 |       |
{LEFT_INDENT}|CCCCCCC|                 |                                   |                 |CCCCCCC|
{LEFT_INDENT}|PPPPPPP| {22:<NAMELEN$}  |                                   |  {61:>NAMELEN$} |PPPPPPP|
{LEFT_INDENT}|       |                                                                       |       |
{LEFT_INDENT}|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|
{LEFT_INDENT}|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|
{LEFT_INDENT}|       |       |       |       |       |       |       |       |       |       |       |
{LEFT_INDENT} ▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔ 
{LEFT_INDENT} {20:^7} {18:^7} {16:^7} {14:^7} {12:^7} {10:^7} {08:^7} {06:^7} {04:^7} {02:^7} {00:^7} 
{LEFT_INDENT} {21:^7} {19:^7} {17:^7} {15:^7} {13:^7} {11:^7} {09:^7} {07:^7} {05:^7} {03:^7} {01:^7} ",
    // Spaces between each name for the top and bottom rows are necessary because we count the width
    // of the tile as the # of space between the | chars, thus excluding them and creating a gap

    // The groups of C chars are to denote the tile's row that will draw its colour
    // The groups of P chars are to denote the tile's row that will show its price and property info
    // These groups MUST be unique for functions to find the substrings and replace them with
    // the actual requested information instead of the placeholders

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
        );

        println!("{}", ascii_board);
        String::from("ascii_board")
    }

    fn add_tile_colours_to_ascii_board(ascii_board: &String) {

    }
}
