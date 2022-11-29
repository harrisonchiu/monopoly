#![allow(dead_code)] // leave this here until done testing
#![allow(unused_variables)]

extern crate const_format;
extern crate serde_json;

use std::collections::HashMap;
type JsonMap = HashMap<String, serde_json::Value>;

mod board;
mod constants;
use board::{BoardTile, PropertyTile};

// println!("\x1B[10;5f2ADSSSSSSSSSSSSSSS");
// println!("\x1B[42;0f");

fn main() {
    let property_json_file = include_str!("./property_data.json");
    let json: serde_json::Value = serde_json::from_str::<serde_json::Value>(&property_json_file)
        .expect("JSON could not be deserialized because of an error, likely has bad format");

    // Skip first JSON object because it is documentation, create board with the rest of it
    // JSON is array of objects so it should preserve order; it should define all the board tiles
    // from GO (start) to Boardwalk (last tile before GO) in order
    for i in json.as_array().unwrap().iter().skip(1) {
        println!("{}", i["name"]);
        // create board from this array but skip first one
    }
    let mediterranean_ave: BoardTile = PropertyTile::new(json["Mediterranean Avenue"].clone());

    //let board: board::Board = board::Board::new([mediterranean_ave]);
    // board.display_board();
}
