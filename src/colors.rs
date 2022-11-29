use const_format::concatcp;
use const_format::str_repeat;

use board::BOARD_TILE_SIZE_BY_CHAR;

// Color is the color background of " ". Not sure why -X, but the text
// without " " is still 5 characters long, likely from \x1b[41m and \x1b[0m
const COLOR_TEXT: &'static str = str_repeat!(" ", BOARD_TILE_SIZE_BY_CHAR);
const END_COLOR: &'static str = concatcp!(COLOR_TEXT, "\x1b[0m");

// pub struct Color;

// impl Color {
//     pub const RED: &'static str = concatcp!("\x1b[41m", COLOR_TEXT, END_COLOR);
//     pub const ORANGE: &'static str = concatcp!("\x1b[48;5;166m", COLOR_TEXT, END_COLOR);
//     pub const YELLOW: &'static str = concatcp!("\x1b[43m", COLOR_TEXT, END_COLOR);

//     pub const GREEN: &'static str = concatcp!("\x1b[42m", COLOR_TEXT, END_COLOR);
//     pub const CYAN: &'static str = concatcp!("\x1b[46m", COLOR_TEXT, END_COLOR);
//     pub const BLUE: &'static str = concatcp!("\x1b[44m", COLOR_TEXT, END_COLOR);
//     pub const MAGENTA: &'static str = concatcp!("\x1b[45m", COLOR_TEXT, END_COLOR);

//     pub const WHITE: &'static str = concatcp!("\x1b[47m", COLOR_TEXT, END_COLOR);
//     pub const GRAY: &'static str = concatcp!("\x1b[100m", COLOR_TEXT, END_COLOR);
//     pub const BROWN: &'static str = concatcp!("\x1b[48;5;94m", COLOR_TEXT, END_COLOR);
// }
