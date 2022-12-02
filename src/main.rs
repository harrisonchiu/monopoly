#![allow(dead_code)] // leave this here until done testing
#![allow(unused_variables)]

extern crate const_format;
extern crate phf;
extern crate serde_json;

mod board;
mod constants;
mod error;
mod tiles;

use tiles::{board_tile, event_tile, railroad_tile, street_tile, utility_tile};

fn main() {
    let property_json_file = include_str!("./tiles/board_tile_data.json");
    let json: serde_json::Value = serde_json::from_str::<serde_json::Value>(&property_json_file)
        .expect("JSON could not be deserialized because of an error, likely has bad format");

    // Skip first JSON object because it is documentation, create board with the rest of it
    // JSON is array of objects so it should preserve order; it should define all the board tiles
    // from GO (start) to Boardwalk (last tile before GO) in order
    let mut board_tiles: Vec<board_tile::BoardTile> = Vec::<board_tile::BoardTile>::new();
    for tile_data in json.as_array().unwrap().iter().skip(1) {
        match tile_data
            .get("type")
            .expect(error::JSON_MISSING_TYPE)
            .as_str()
            .expect(error::JSON_DESERIALIZE_TO_STR)
        {
            "Street" => board_tiles.push(board_tile::BoardTile::StreetTile(
                street_tile::StreetTile::new(tile_data.clone()),
            )),
            "Railroad" => board_tiles.push(board_tile::BoardTile::RailroadTile(
                railroad_tile::RailroadTile::new(tile_data.clone()),
            )),
            "Utility" => board_tiles.push(board_tile::BoardTile::UtilityTile(
                utility_tile::UtilityTile::new(tile_data.clone()),
            )),
            "Event" => board_tiles.push(board_tile::BoardTile::EventTile(
                event_tile::EventTile::new(tile_data.clone()),
            )),
            _ => continue,
        }
    }

    let board: board::Board = board::Board::new(board_tiles);
    board.display_board();
}
