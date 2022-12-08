extern crate const_format;
extern crate phf;
extern crate rand;
extern crate serde_json;

mod board;
mod display;
mod error;
mod game;
mod player;
mod tiles;

use std::io;

use rand::distributions::Uniform;
use rand::{rngs::StdRng, SeedableRng};

use tiles::board_tile::BoardTile;

fn prompt(avatar: &String) -> Option<i32> {
    display::move_cursor_to_input();
    print!("[Player {}] >>> ", avatar); // the input prompt
    display::flush_buffer();

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    input_text.trim().parse::<i32>().ok()
}

fn main() {
    // Set up the game: dice, players, board
    let die_range: Uniform<i8> = Uniform::new_inclusive(1, 6);
    let mut die_1: StdRng = StdRng::from_entropy();
    let mut die_2: StdRng = StdRng::from_entropy();

    let mut players: Vec<player::Player> = Vec::<player::Player>::with_capacity(4);
    for (id, avatar) in game::PLAYER_PIECES.iter().enumerate() {
        players.push(player::Player::new(id, avatar.to_string()));
    }

    let mut board: board::Board = game::create_board();

    // Display the game in the terminal
    // Show board before the players - display_board() makes the basis of where to display
    board.display_board();
    players.iter().for_each(|player| player.display_at_start(0));

    // Assume players::Vec will not remove any items, so players[i] is guaranteed to succeed
    let mut is_dice_rollable: bool = true;
    for i in (0..players.len()).cycle() {
        display::terminal_bell();
        display::flush_buffer();
        display::inform(game::INSTRUCTIONS_MAIN_MENU_ROLL);

        while let Some(command) = prompt(&players[i].avatar) {
            if command == 1 && is_dice_rollable {
                // Roll and Move
                let dice: [i8; 2] = game::roll_dice(&die_range, &mut die_1, &mut die_2);
                is_dice_rollable = game::is_doubles(&dice);

                (&mut players[i]).move_forwards(dice[0] + dice[1]);
                let landed_tile: &BoardTile = board.get_tile(players[i].position);

                // Check if the roll is double and let the player roll again if so
                if is_dice_rollable {
                    display::output(format!(
                        "[*] DOUBLES! You rolled {}, {:?}, onto {}.",
                        dice[0] + dice[1],
                        dice,
                        landed_tile.get_tile_name()
                    ));
                } else {
                    display::inform(game::INSTRUCTIONS_MAIN_MENU_END_TURN);
                    display::output(format!(
                        "[*] You rolled {}, {:?}, onto {}.",
                        dice[0] + dice[1],
                        dice,
                        landed_tile.get_tile_name()
                    ));
                }

                // Player automatically pays rent to landlord and append the log
                if let Some(landlord) = landed_tile.get_owner_id() {
                    let rent: i64 = landed_tile.get_rent(dice[0] + dice[1]);
                    (&mut players[i]).pay(rent);
                    (&mut players[landlord]).collect(rent);
                    print!(
                        " You paid ${rent} to Player {} for rent.",
                        &players[landlord].avatar
                    );
                }
            } else if command == 1 && !is_dice_rollable {
                // End Turn - current player is finished and next player goes
                display::output("");
                is_dice_rollable = true;
                break;
            } else if command == 2 {
                // Buy Property
                let tile: &mut BoardTile = board.get_tile_mut(players[i].position);
                if let Some(cost) = game::try_to_buy_tile(&mut players[i], tile) {
                    display::output(format!(
                        "[*] Purchased tile for ${cost}. Amount of money remaining ${}.",
                        players[i].money
                    ));
                } else {
                    if let Some(landlord) = tile.get_owner_id() {
                        display::output(format!(
                            "[!] Cannot purchase tile! Tile is owned by Player {}.",
                            &players[landlord].avatar
                        ));
                    } else {
                        // If tile cannot be purchased and has no owner, it is an event tile
                        display::output("[!] Cannot purchase tile! Tile is unownable.");
                    }
                }
            } else {
                display::output("")
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
