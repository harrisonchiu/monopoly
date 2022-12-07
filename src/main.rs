#![allow(dead_code)] // leave this here until done testing
#![allow(unused_variables)]

extern crate const_format;
extern crate phf;
extern crate rand;
extern crate serde_json;

mod board;
mod constants;
mod display;
mod error;
mod player;
mod tiles;

use std::io;

use rand::distributions::{Distribution, Uniform};
use rand::{rngs::StdRng, SeedableRng};

use tiles::{
    board_tile::BoardTile, event_tile::EventTile, railroad_tile::RailroadTile,
    street_tile::StreetTile, utility_tile::UtilityTile,
};

fn prompt() -> i32 {
    display::move_cursor_to_input();
    print!(">>> "); // the input prompt
    display::flush_buffer();

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    input_text.trim().parse::<i32>().unwrap_or(0)
}

fn create_board() -> board::Board {
    let tile_data_json: &str = include_str!("./tiles/board_tile_data.json");
    let json: serde_json::Value = serde_json::from_str::<serde_json::Value>(&tile_data_json)
        .expect("JSON could not be deserialized because of an error, likely has bad format");

    // Skip first JSON object because it is documentation and metadata, create board
    // with the rest of it. JSON is array of objects so it should preserve order; it should
    // define all the board tiles from GO (start) to Boardwalk (last tile before GO) in order
    let mut tiles: Vec<BoardTile> = Vec::<BoardTile>::new();
    for tile_data in json.as_array().unwrap().iter().skip(1) {
        match tile_data
            .get("type")
            .expect(error::JSON_MISSING_TYPE)
            .as_str()
            .expect(error::JSON_DESERIALIZE_TO_STR)
        {
            "Street" => tiles.push(BoardTile::Street(StreetTile::new(tile_data.clone()))),
            "Railroad" => tiles.push(BoardTile::Railroad(RailroadTile::new(tile_data.clone()))),
            "Utility" => tiles.push(BoardTile::Utility(UtilityTile::new(tile_data.clone()))),
            "Event" => tiles.push(BoardTile::Event(EventTile::new(tile_data.clone()))),
            _ => continue,
        }
    }

    board::Board::new(tiles)
}

fn roll_dice(die_range: &Uniform<u8>, die_1: &mut StdRng, die_2: &mut StdRng) -> [u8; 2] {
    // We split it up to make it more extendible in the future
    // That is we could have an option to use a singular die [0, 12] for an equal
    // distribution (which would change the game a lot) and make it easier to
    // display the value of each die (draw each die value instead of just printing the sum)
    [die_range.sample(die_1), die_range.sample(die_2)]
}

fn is_doubles(dice: [u8; 2]) -> bool {
    dice[0] == dice[1]
}

fn main() {
    let die_range: Uniform<u8> = Uniform::new_inclusive(1, 6);
    let mut die_1: StdRng = StdRng::from_entropy();
    let mut die_2: StdRng = StdRng::from_entropy();

    let mut players: Vec<player::Player> = Vec::<player::Player>::with_capacity(4);
    for (id, avatar) in ['A', 'B', 'C', 'D'].iter().enumerate() {
        players.push(player::Player::new(id as u8, *avatar));
    }

    let board: board::Board = create_board();

    // Display the game in the terminal
    // Show board before the players - display_board() makes the basis of where to display
    board.display_board();
    players.iter().for_each(|player| player.display_at_start(0));

    // Assume players::Vec will not remove any items, so players[i] is guaranteed to succeed
    let mut is_player_finished_rolling: bool = false;
    for i in (0..players.len()).cycle() {
        display::flush_buffer();
        display::inform(constants::INSTRUCTIONS_MAIN_MENU_ROLL);
        loop {
            match prompt() {
                1 if !is_player_finished_rolling => {
                    // Roll and Move
                    let dice: [u8; 2] = roll_dice(&die_range, &mut die_1, &mut die_2);
                    let dice_total: u8 = dice[0] + dice[1];
                    players[i].move_forwards(dice_total);
                    display::output(format!("Rolled {dice_total} ({}, {})", dice[0], dice[1]));

                    match is_doubles(dice) {
                        true => is_player_finished_rolling = false,
                        false => {
                            // Player cannot roll anymore
                            display::inform(constants::INSTRUCTIONS_MAIN_MENU_END_TURN);
                            is_player_finished_rolling = true;
                        }
                    }
                }
                1 if is_player_finished_rolling => {
                    // End Turn - current player is finished and next player goes
                    is_player_finished_rolling = false;
                    break;
                }
                2 => {
                    // Buy Property
                    //board.get_tile(players[i].position)
                }
                _ => todo!(),
            }
        }
    }

    /*
    Actions: quit/save, ff, view specific property, roll and move, end turn, pay rent, buy property,
    trade, buy buildings, sell/mortgage, auction

    fuck it
        we auto pay rent
        trade
            => new menu
                => select who to trade, back
        view specific property
            => show board tile indices on board and ask for index input
                => buy buildings, sell/mortgage, back
        end turn (only if roll/move has been made otherwise it returns to strerr)
            => auto auctions if gameplay setting for that auction rule is on
    therefore

    actions (your turn)
        quit/save, ff, view specific property, roll/move, end turn (if roll/move has been made),
        buy property (if property tile and unowned), trade
    actions (not your turn and he finished his turn)
        quit/save, ff, view specific property (without submenu of buying buildings, sell/mortgage),
        let him end his turn (he already rolled, etc, and did everything he wanted),
        trade (technically we should allow trade before he makes his move
            and after he makes his move, but it is effectively redundant)

    myturn, trade | histurn, trade | histurn, trade |
    */
}
