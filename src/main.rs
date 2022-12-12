extern crate const_format;
extern crate phf;
extern crate rand;
extern crate serde_json;

mod board;
mod colours;
mod display;
mod error;
mod game;
mod interface;
mod player;
mod tiles;

use std::io;

use game::Monopoly;
use player::Pieces;

enum Action {
    Move = 1,
    BuyProperty = 2,
    Redraw = 99,
}

type CommandId = i32;
fn prompt(id: usize) -> Option<CommandId> {
    display::move_cursor_to_input();
    print!(
        "[{}Player {}\x1b[0m] >>> ",
        Pieces::colour(&id),
        Pieces::avatar(&id)
    );
    display::flush_buffer();
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    if let Ok(input) = input_text.trim().parse::<CommandId>() {
        return Some(input);
    }
    match input_text.trim() {
        "redraw" => Some(Action::Redraw as CommandId),
        _ => None,
    }
}

fn main() {
    const NUMBER_OF_PLAYERS: usize = 4;

    let mut game: Monopoly = Monopoly::new(NUMBER_OF_PLAYERS);
    game.display_game();

    let mut is_dice_rollable: bool = true;
    for id in (0..NUMBER_OF_PLAYERS).cycle() {
        display::terminal_bell();
        display::inform(interface::INSTRUCTIONS_MOVE);
        display::flush_buffer();

        while let Some(command) = prompt(id) {
            if command == Action::Move as CommandId && is_dice_rollable {
                let dice: [i8; 2] = game.roll_dice();
                let tile: usize = game.move_player(id, &dice);
                let landlord: Option<usize> = game.get_owner(tile);
                let rent: i64 = game.get_rent(tile, &dice);

                is_dice_rollable = game.is_doubles(&dice);
                if is_dice_rollable {
                    display::output("[*] DOUBLES!! ");
                } else {
                    display::inform(interface::INSTRUCTIONS_END_TURN);
                    display::output("[*] ");
                }

                print!("Rolled {dice:?}, landing on {}. ", game.get_tile_name(tile));
                if let Some(landlord) = landlord {
                    if landlord == id {
                        break;
                    }

                    game.pay_rent_to(id, landlord, rent);
                    print!(
                        "Paid ${rent} in rent to {}Player {}\x1b[0m. ",
                        Pieces::colour(&landlord),
                        Pieces::avatar(&landlord)
                    );
                }

                // game.buy_tile(id, &players[id].colour, players[id].position);
            } else if command == Action::Move as CommandId && !is_dice_rollable {
                display::clear_output();
                is_dice_rollable = true;
                break;
            } else if command == Action::BuyProperty as CommandId {
                let tile: usize = game.get_player(id).position;

                if let Some(purchased_tile) = game.buy_tile(id, tile) {
                    display::output(format!(
                        "[*] Purchased {} for ${}. Amount of money remaining ${}.",
                        game.get_tile_name(purchased_tile),
                        game.get_property_cost(purchased_tile),
                        game.get_player(id).money
                    ));
                } else if let Some(landlord) = game.get_owner(tile) {
                    display::output(format!(
                        "[!] Cannot purchase tile! {} is already owned by {}Player {}\x1b[0m.",
                        game.get_tile_name(tile),
                        &Pieces::colour(&landlord),
                        &Pieces::avatar(&landlord)
                    ));
                } else {
                    display::output("[!] Cannot purchase tile! Tile cannot be owned!");
                }
            } else if command == Action::Redraw as CommandId {
                display::clear_display();
                display::flush_buffer();
                game.display_game();
            } else {
                display::output("SSSSS");
            }
        }
    }
}
