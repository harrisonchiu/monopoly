extern crate const_format;
extern crate phf;
extern crate rand;
extern crate serde_json;

mod board;
mod colours;
mod display;
mod error;
mod game;
mod player;
mod tiles;

use std::io;

use game::Monopoly;
use player::Player;

enum Action {
    Move = 1,
    BuyProperty = 2,
    Redraw = 99,
}

fn prompt(colour: &String, avatar: &char) -> Option<i32> {
    display::move_cursor_to_input();
    print!("[{}Player {}\x1b[0m] >>> ", colour, avatar);
    display::flush_buffer();
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    if let Ok(input) = input_text.trim().parse::<i32>() {
        return Some(input);
    }
    match input_text.trim() {
        "redraw" => Some(Action::Redraw as i32),
        _ => None,
    }
}

fn main() {
    const NUMBER_OF_PLAYERS: usize = 4;

    let mut players: Vec<Player> = Vec::<Player>::with_capacity(NUMBER_OF_PLAYERS);
    for (id, piece) in player::PLAYER_PIECES.iter().enumerate() {
        players.push(Player::new(id, piece.0, piece.1.to_string()));
    }

    let mut game: Monopoly = Monopoly::new(NUMBER_OF_PLAYERS);
    game.display_game();
    players.iter().for_each(|p| p.display_at_position(0));

    let mut is_dice_rollable: bool = true;
    for id in (0..NUMBER_OF_PLAYERS).cycle() {
        display::terminal_bell();
        display::flush_buffer();

        while let Some(command) = prompt(&players[id].colour, &players[id].avatar) {
            if command == Action::Move as i32 && is_dice_rollable {
                let dice: [i8; 2] = game.roll_dice();
                let tile: usize = (&mut players[id]).walk(&dice[0] + &dice[1]);
                let landlord: Option<usize> = game.get_landlord(tile);
                let rent: i64 = game.get_rent(tile, &dice);

                display::output("[*] ");
                is_dice_rollable = game.is_doubles(&dice);
                if is_dice_rollable {
                    print!("DOUBLES!! ");
                }

                if landlord.is_none() || landlord.is_some() && landlord.unwrap() == id {
                    print!(
                        "Rolled {dice:?} and landed on {}.",
                        game.get_tile_name(tile),
                    );
                } else if let Some(landlord) = landlord {
                    players[id].pay(rent);
                    players[landlord].collect(rent);
                    print!(
                        "Rolled {dice:?} and landed on {}. Paid ${rent} in rent to {}Player {}\x1b[0m.",
                        game.get_tile_name(tile),
                        &players[landlord].colour,
                        &players[landlord].avatar
                    );
                }

                game.buy_tile(id, &players[id].colour, players[id].position);
            } else if command == Action::Move as i32 && !is_dice_rollable {
                display::clear_output();
                is_dice_rollable = true;
                break;
            } else if command == Action::BuyProperty as i32 {
                if let Some(cost) = game.buy_tile(id, &players[id].colour, players[id].position) {
                    players[id].pay(cost);
                    display::output(format!(
                        "[*] Purchased tile for ${cost}. Amount of money remaining ${}.",
                        players[id].money
                    ));
                } else if let Some(landlord) = game.get_landlord(players[id].position) {
                    display::output(format!(
                        "[!] Cannot purchase tile! Tile is already owned by {}Player {}\x1b[0m.",
                        &players[landlord].colour, &players[landlord].avatar
                    ));
                } else {
                    display::output("[!] Cannot purchase tile! Tile cannot be owned!");
                }
            } else if command == Action::Redraw as i32 {
                display::clear_display();
                display::flush_buffer();
                game.display_game();
                players
                    .iter()
                    .for_each(|p| p.display_at_position(p.position));
            } else {
                display::output("SSSSS");
            }
        }
    }
}
