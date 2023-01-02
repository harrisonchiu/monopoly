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
    Move,
    BuyProperty,
    ViewIndex,
    ViewProperty,

    Trade,
    TradeStart,
    GiveTile,
    GiveMoney,
    TakeTile,
    TakeMoney,
    MinePop,
    TheirPop,
    Propose,
    Reject,
    Accept,
    Abort,

    Redraw,
    None,
    Error,
}

fn prompt(id: usize) -> (Action, Action, i64) {
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

    let mut args: [&str; 4] = ["", "", "", ""];
    let mut number_argument: i64 = -99;
    for (i, arg) in input_text.split_whitespace().enumerate().take(args.len()) {
        args[i] = arg;
        number_argument = arg.parse::<i64>().ok().unwrap_or_else(|| -99);
    }

    match args[0] {
        "roll" | "r" => (Action::Move, Action::None, -99),
        "buy" | "b" => (Action::BuyProperty, Action::None, -99),
        "view" | "v" => {
            if number_argument >= 0 {
                (Action::ViewProperty, Action::None, number_argument)
            } else {
                (Action::ViewIndex, Action::None, -99)
            }
        }
        "trade" | "t" => {
            if args[1] == "start" {
                (Action::Trade, Action::TradeStart, number_argument)
            } else if (args[1], args[2]) == ("give", "tile") {
                (Action::Trade, Action::GiveTile, number_argument)
            } else if (args[1], args[2]) == ("give", "money") {
                (Action::Trade, Action::GiveMoney, number_argument)
            } else if (args[1], args[2]) == ("take", "tile") {
                (Action::Trade, Action::TakeTile, number_argument)
            } else if (args[1], args[2]) == ("take", "money") {
                (Action::Trade, Action::TakeMoney, number_argument)
            } else if (args[1], args[2]) == ("mine", "pop") {
                (Action::Trade, Action::MinePop, number_argument)
            } else if (args[1], args[2]) == ("their", "pop") {
                (Action::Trade, Action::TheirPop, number_argument)
            } else if args[1] == "propose" {
                (Action::Trade, Action::Propose, -99)
            } else if args[1] == "accept" {
                (Action::Trade, Action::Accept, -99)
            } else if args[1] == "reject" {
                (Action::Trade, Action::Reject, -99)
            } else if args[1] == "abort" {
                (Action::Trade, Action::Abort, -99)
            } else {
                (Action::Trade, Action::None, -99)
            }
        }
        "redraw" => (Action::Redraw, Action::None, -99),
        _ => (Action::Error, Action::Error, -99),
    }
}

fn main() {
    const NUMBER_OF_PLAYERS: usize = 2;

    let mut game: Monopoly = Monopoly::new(NUMBER_OF_PLAYERS);
    game.display_game();

    let mut is_dice_rollable: bool = true;
    let mut is_trade_started: bool = false;
    let mut view_menu: bool = true;
    for id in (0..NUMBER_OF_PLAYERS).cycle() {
        display::terminal_bell();
        display::inform(interface::INSTRUCTIONS_MOVE);
        display::flush_buffer();

        #[allow(irrefutable_let_patterns)]
        while let (command, subcommand, number) = prompt(id) {
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
                print!(
                    "Rolled {dice:?}, landing on {}. ",
                    &game.get_tile_name(tile)
                );

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
                        &game.get_tile_name(purchased_tile),
                        game.get_property_cost(purchased_tile),
                        game.get_player(id).money
                    ));
                } else if let Some(landlord) = game.get_owner(tile) {
                    display::output(format!(
                        "[!] Cannot purchase tile! {} is already owned by {}Player {}\x1b[0m.",
                        &game.get_tile_name(tile),
                        &Pieces::colour(&landlord),
                        &Pieces::avatar(&landlord)
                    ));
                } else {
                    display::output("[!] Cannot purchase tile! Tile cannot be owned!");
                }
            } else if command == Action::ViewIndex {
                match view_menu {
                    true => game.view_tile_ids(),
                    false => game.display_players(),
                }
                view_menu = !view_menu;
            } else if command == Action::ViewProperty {
                game.display_full_tile_info(number as usize);
            } else if command == Action::Trade {
                interface::clear_inside_board();
                if NUMBER_OF_PLAYERS <= 1 {
                    continue;
                }

                match subcommand {
                    Action::TradeStart if !is_trade_started => {
                        if (0..NUMBER_OF_PLAYERS).contains(&(number as usize))
                            && id != number as usize
                        {
                            game.trade_start(id, number as usize);
                            is_trade_started = true;
                            display::output("[*] Started trade");
                        } else if id == number as usize {
                            display::output("[!] Cannot trade with yourself!");
                        } else {
                            display::output("[!] Unknown player ID");
                        }
                    }
                    Action::TradeStart if is_trade_started => {
                        display::output("[!] Already started trade!");
                    }
                    Action::GiveTile if is_trade_started => {
                        if game.trade_give_tile(number as usize) {
                            display::output(&format!("[*] Offered tile id {number}"));
                        } else {
                            display::output(&format!("[!] You do not own tile id {number}"));
                        }
                    }
                    Action::GiveMoney if is_trade_started => {
                        if game.trade_give_money(number) {
                            display::output(&format!("[*] Offered ${number}"));
                        } else {
                            display::output(&format!("[!] You do not have ${number}"));
                        }
                    }
                    Action::TakeTile if is_trade_started => {
                        if game.trade_take_tile(number as usize) {
                            display::output(&format!(
                                "[*] Asked for tile id {number} from receiver"
                            ));
                        } else {
                            display::output(&format!("[!] Receiver does not own tile id {number}"));
                        }
                    }
                    Action::TakeMoney if is_trade_started => {
                        if game.trade_take_money(number) {
                            display::output(&format!("[*] Asked for ${number} from receiver"));
                        }
                        display::output(&format!("[!] Receiver does not have ${number}"));
                    }
                    Action::MinePop if is_trade_started => {
                        if game.trade_mine_pop(number) {
                            display::output(&format!(
                                "[*] Removed trade item id {number} from proposer's offer"
                            ));
                        } else {
                            display::output(&format!(
                                "[*] Unknown item id {number} in proposer's offer"
                            ));
                        }
                    }
                    Action::TheirPop if is_trade_started => {
                        if game.trade_their_pop(number) {
                            display::output(&format!(
                                "[*] Removed trade item id {number} from receiver's offer"
                            ));
                        } else {
                            display::output(&format!(
                                "[*] Unknown item id {number} in receiver's offer"
                            ));
                        }
                    }
                    Action::Propose if is_trade_started => loop {
                        game.display_trading_desk();
                        display::output(
                            "[*] Switch to receiver, \
                        so they can choose to accept or reject the trade",
                        );
                        match prompt(game.receiver) {
                            (Action::Trade, Action::Accept, -99) => {
                                game.accept_trade();
                                display::output("[*] Trade accepted!");
                                is_trade_started = false;
                                break;
                            }
                            (Action::Trade, Action::Reject, -99) => {
                                game.reject_trade();
                                display::output("[*] Trade rejected!");
                                is_trade_started = false;
                                break;
                            }
                            _ => display::output("[!] Only commands: trade accept | trade reject"),
                        }
                    },
                    Action::Abort if is_trade_started => {
                        game.clear_trade();
                        interface::clear_inside_board();
                        is_trade_started = false;
                        display::output("[*] Trade has been aborted");
                    }
                    Action::None if !is_trade_started => {
                        display::output(format!("[*] Trade with someone: {}", Pieces::view_ids()));
                    }
                    Action::None if is_trade_started => {
                        display::output(
                            "[*] Trade commands: trade \
                        {start X|give/take tile/money X|my/their pop X|propose/reject/abort}",
                        );
                    }
                    // An actual trade command ran but a trade has not been started
                    // so the game does not know which players to fetch info from
                    _ if !is_trade_started => {
                        display::output(
                            "[!] You must start a trade with someone first! \
                            Run: trade start <PLAYER_ID>",
                        );
                    }
                    _ => display::output("[!] Unknown trade command"),
                }
                if is_trade_started {
                    game.display_trading_desk();
                }

                game.update_inventory_display();
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
