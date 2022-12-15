#![allow(unused_doc_comments)]
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

#[derive(PartialEq)]
enum Action {
    /// (-inf, -1] - Error codes
    /// [0, 99] - Actual numbers used to
    /// 1XX - Normal game actions
    /// 2XX - Debug actions
    ///
    Move = 1,
    BuyProperty = 2,
    ViewProperty = 103,
    // when viewing tiles does
    // not conflict with inputting commands
    Redraw = 99,
    Error = -99,
}

fn get_number_argument_from_command<'a>(input: &'a String) -> Option<i32> {
    //! A number command is a 2 argument &str user input that is a directive.
    //! The number will always follow the string command.
    //! A number command is of the format: "<SINGLE_COMMAND_WORD> <ONE_NUMBER>"
    //! Ex: let number_command: &str = "view 39";
    if let Ok(number_argument) = input
        .split_whitespace()
        .nth(1)
        .unwrap_or_default()
        .parse::<i32>()
    {
        Some(number_argument)
    } else {
        None
    }
}

fn prompt(id: usize) -> (Action, Option<i32>) {
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
        .expect("Failed to read from stdin!");

    let number_argument: Option<i32> = get_number_argument_from_command(&input_text);

    match input_text.trim() {
        "move" | "roll" | "end" | "r" => (Action::Move, None),
        "buy" | "b" => (Action::BuyProperty, None),
        "view" | "v" => (Action::ViewProperty, None),
        "redraw" => (Action::Redraw, None),
        command if command.starts_with("view") && number_argument.is_some() => {
            display::debug("SDFSDFSDFSDf");
            (Action::ViewProperty, number_argument)
        }
        /// need to view property by index (view id)
        /// need to trade money by numbers (give/take money)
        /// need to bid by giving numbers (auction price)
        /// maybe the numbers can be suffixed for these commands and
        /// those non number commands will just give status code like http status
        _ => (Action::Error, Some(-99)),
    }
}

fn main() {
    const NUMBER_OF_PLAYERS: usize = 1;

    let mut game: Monopoly = Monopoly::new(NUMBER_OF_PLAYERS);
    game.display_game();

    let mut is_dice_rollable: bool = true;
    let mut view_menu: bool = true;
    for id in (0..NUMBER_OF_PLAYERS).cycle() {
        display::terminal_bell();
        display::inform(interface::INSTRUCTIONS_MOVE);
        display::flush_buffer();

        #[allow(irrefutable_let_patterns)]
        while let (command, number) = prompt(id) {
            if command == Action::Move && is_dice_rollable {
                let dice: [i8; 2] = game.roll_dice();
                let tile: usize = game.move_player(id, &dice);

                is_dice_rollable = game.is_doubles(&dice);
                if is_dice_rollable {
                    display::output("[*] DOUBLES!! ");
                } else {
                    display::inform(interface::INSTRUCTIONS_END_TURN);
                    display::output("[*] ");
                }
                print!("Rolled {dice:?}, landing on {}. ", game.get_tile_name(tile));

                let rent: i64 = game.get_rent(tile, &dice);
                if let Some(landlord) = game.get_owner(tile) {
                    if landlord == id {
                        continue;
                    }

                    game.pay_rent(id, landlord, rent);
                    print!(
                        "Paid ${rent} in rent to {}Player {}\x1b[0m. ",
                        Pieces::colour(&landlord),
                        Pieces::avatar(&landlord)
                    );
                }

                game.update_inventory_display();
            } else if command == Action::Move && !is_dice_rollable {
                display::clear_output();
                is_dice_rollable = true;
                break;
            } else if command == Action::BuyProperty {
                let tile: usize = game.get_player(id).position;

                if let Some(purchased_tile) = game.buy_tile(id, tile) {
                    game.update_inventory_display();
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
            } else if command == Action::ViewProperty && number.is_none() {
                match view_menu {
                    true => game.view_tile_ids(),
                    false => game.display_players(),
                }
                view_menu = !view_menu;
            } else if command == Action::ViewProperty && number.is_some() {
                game.display_full_tile_info(number.unwrap() as usize);
            } else if command == Action::Redraw {
                display::clear_display();
                display::flush_buffer();
                game.display_game();
            } else {
                display::output("SSSSS");
            }
        }
    }
}
