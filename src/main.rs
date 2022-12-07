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
mod game;
mod player;
mod tiles;

use std::io;

use rand::distributions::Uniform;
use rand::{rngs::StdRng, SeedableRng};

use tiles::board_tile::BoardTile;

fn prompt(avatar: &String) -> i32 {
    display::move_cursor_to_input();
    print!("[Player {}] >>> ", avatar); // the input prompt
    display::flush_buffer();

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    input_text.trim().parse::<i32>().unwrap_or(0)
}

fn main() {
    // Set up the game: dice, players, board
    let die_range: Uniform<u8> = Uniform::new_inclusive(1, 6);
    let mut die_1: StdRng = StdRng::from_entropy();
    let mut die_2: StdRng = StdRng::from_entropy();

    let mut players: Vec<player::Player> = Vec::<player::Player>::with_capacity(4);
    for (id, avatar) in constants::PLAYER_PIECES {
        players.push(player::Player::new(id, avatar.to_string()));
    }

    let mut board: board::Board = game::create_board();

    // Display the game in the terminal
    // Show board before the players - display_board() makes the basis of where to display
    board.display_board();
    players.iter().for_each(|player| player.display_at_start(0));

    // Assume players::Vec will not remove any items, so players[i] is guaranteed to succeed
    let mut player: &mut player::Player;
    let mut is_player_finished_rolling: bool = false;
    for i in (0..players.len()).cycle() {
        display::flush_buffer();
        display::inform(constants::INSTRUCTIONS_MAIN_MENU_ROLL);
        player = &mut players[i];
        loop {
            match prompt(&player.avatar) {
                1 if !is_player_finished_rolling => {
                    // Roll and Move
                    let dice: [u8; 2] = game::roll_dice(&die_range, &mut die_1, &mut die_2);
                    display::flush_buffer();
                    player.move_forwards(dice[0] + dice[1]);
                    display::flush_buffer();

                    is_player_finished_rolling = !game::is_doubles(&dice);
                    if is_player_finished_rolling {
                        display::inform(constants::INSTRUCTIONS_MAIN_MENU_END_TURN);
                        display::output(format!(
                            "[*] You rolled {0} ({1} and {2}) onto {3}.",
                            dice[0] + dice[1],
                            dice[0],
                            dice[1],
                            board.get_tile_name_from_position(player.position)
                        ));
                    } else {
                        display::output(format!(
                            "[*] DOUBLES! You rolled {0} ({1} and {2}) onto {3}.",
                            dice[0] + dice[1],
                            dice[0],
                            dice[1],
                            board.get_tile_name_from_position(player.position)
                        ));
                    }
                }
                1 if is_player_finished_rolling => {
                    // End Turn - current player is finished and next player goes
                    display::output("[*] You have ended your turn");
                    is_player_finished_rolling = false;
                    break;
                }
                2 => {
                    // Buy Property
                    let tile: &mut BoardTile = board.get_tile(player.position);
                    if let Some(price) = game::purchase_tile(&mut player, tile) {
                        display::output(format!(
                            "[*] Purchased tile for ${}. Amount of money remaining ${}.",
                            price, player.money
                        ));
                    } else {
                        display::output("[!] Cannot purchase tile!");
                    }
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
