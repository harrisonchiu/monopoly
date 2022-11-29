#![allow(dead_code)] // leave this here until done testing
#![allow(unused_variables)]

extern crate const_format;
extern crate phf;
extern crate serde_json;

mod board;
mod colours;
mod constants;

// println!("\x1B[10;5f2ADSSSSSSSSSSSSSSS");
// println!("\x1B[42;0f");

fn main() {
    let property_json_file = include_str!("./property_data.json");
    let json: serde_json::Value = serde_json::from_str::<serde_json::Value>(&property_json_file)
        .expect("JSON could not be deserialized because of an error, likely has bad format");

    // Skip first JSON object because it is documentation, create board with the rest of it
    // JSON is array of objects so it should preserve order; it should define all the board tiles
    // from GO (start) to Boardwalk (last tile before GO) in order
    let mut board_tiles: Vec<board::BoardTile> = Vec::<board::BoardTile>::new();
    for tile_data in json.as_array().unwrap().iter().skip(1) {
        board_tiles.push(board::PropertyTile::new(tile_data.clone()));
    }

    let board: board::Board = board::Board::new(board_tiles);
    board.display_board();
}
