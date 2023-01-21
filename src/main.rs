#![allow(unused_doc_comments)]
extern crate dynfmt;
extern crate itertools;
extern crate serde_json;

mod board;
mod board_tiles;
mod colour;
mod display;
mod player;

use board_tiles::{
    go::Go, railroad::Railroad, street::Street, tile::Event, tile::Property, tile::Tile,
};

fn main() {
    let tile_data_json: &str = include_str!("./board_tile_data.json");
    let json: serde_json::Value = serde_json::from_str(&tile_data_json).unwrap();

    let mut board: Vec<Tile> = Vec::with_capacity(40);
    for (tile_id, tile_data) in json.as_array().unwrap().iter().enumerate() {
        match tile_data["type"].as_str().unwrap() {
            "Street" => {
                board.push(Tile::Property(Box::new(Street::new(tile_id, tile_data))));
            }
            "Railroad" => {
                board.push(Tile::Property(Box::new(Railroad::new(tile_id, tile_data))));
            }
            "Utility" => {
                board.push(Tile::Property(Box::new(Railroad::new(tile_id, tile_data))));
            }
            "Event" => {
                board.push(Tile::Event(Box::new(Go::new(tile_id, tile_data))));
            }
            _ => continue,
        }
    }

    const NUMBER_OF_PLAYERS: usize = 4;
    let mut terminal = display::Terminal::new(&board);
    let mut players: Vec<player::Player> = Vec::with_capacity(4);

    terminal.display_board();
    for id in 0..NUMBER_OF_PLAYERS {
        players.push(player::Player::new(id as i8));
        terminal.display_player(&players[id]);
    }

    'players: for id in (0..NUMBER_OF_PLAYERS).cycle() {
        'commands: while let Some(command) = terminal.prompt(id as i8) {
            if command == "end" {
                break 'commands;
            }
            if command == "quit" {
                break 'players;
            }

            players[id].walk(1);
            terminal.display_player(&players[id]);
        }
    }

    display::Terminal::move_cursor(40, 1);
}
