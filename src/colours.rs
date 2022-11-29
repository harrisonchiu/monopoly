use const_format;
use phf;

use constants;

// Colour the background of ▔ so it looks like the tile has a colour set
// but is still contained within the tile (by the ▔ character)
// NOTE: we do not colour the foreground; different terminals may use various
// foreground colours. We assume white (or similar) foreground for contrast.
// This may be a problem in the future.
const COLOUR_TEXT: &'static str = const_format::str_repeat!("▔", constants::TILE_LENGTH_BY_CHAR);
const END_COLOR: &'static str = "\x1b[0m";

// We use our own str instead of a external crate (e.g. ansi_term). Those usually uses
// a variety of structs and cannot be used inside static functions. By creating our own
// barebones ANSI background colour codes, we can create this string lookup table at compile time
// As mentioned in above NOTE, this may be a problem if we need to ensure white foreground colour
pub static SET_NAME_TO_BACKGROUND_COLOUR: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "Red" => const_format::concatcp!("\x1b[41m", COLOUR_TEXT, END_COLOR),
    "Orange" => const_format::concatcp!("\x1b[48;5;166m", COLOUR_TEXT, END_COLOR),
    "Yellow" => const_format::concatcp!("\x1b[43m", COLOUR_TEXT, END_COLOR),
    "Green" => const_format::concatcp!("\x1b[42m", COLOUR_TEXT, END_COLOR),
    "Cyan" => const_format::concatcp!("\x1b[46m", COLOUR_TEXT, END_COLOR),
    "Blue" => const_format::concatcp!("\x1b[44m", COLOUR_TEXT, END_COLOR),
    "Magenta" => const_format::concatcp!("\x1b[45m", COLOUR_TEXT, END_COLOR),
    "Utilities" => const_format::concatcp!("\x1b[47m",  COLOUR_TEXT, END_COLOR), // White
    "Railroad" => const_format::concatcp!("\x1b[100m",  COLOUR_TEXT, END_COLOR), // Gray
    "Brown" => const_format::concatcp!("\x1b[48;5;94m", COLOUR_TEXT, END_COLOR),
};
