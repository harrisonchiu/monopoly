#![allow(unused_doc_comments)]
extern crate dynfmt;
extern crate itertools;
extern crate serde_json;

mod board;
mod tile;

fn main() {
    let tile_data_json: &str = include_str!("./board_tile_data.json");
    let json: serde_json::Value =
        serde_json::from_str(&tile_data_json).expect(&format!("{}", "SDFSDFSDF"));

    let mut tiles: Vec<tile::Tile> = Vec::with_capacity(40);
    for tile_data in json.as_array().unwrap().iter() {
        tiles.push(tile::Tile::new(tile_data));
    }

    let s = board::Board::new(tiles);
}
