use board;
use colours;
use game::PropertySet;
use player::Player;
use std::collections::BTreeMap;
use tiles::board_tile::BoardTile;

pub const INSTRUCTIONS_MOVE: &str =
    "    | [r] Roll/Move | [b] Buy Property | [v] View a Property | [4] Trade | [5] Quit |";
pub const INSTRUCTIONS_END_TURN: &str =
    "    | [r] End Turn | [b] Buy Property | [v] View a Property | [4] Trade | [5] Quit |";

pub fn display_inventory(ownership_records: &Vec<PropertySet>, players: &Vec<Player>) {
    const START_LEFT: usize = board::BOARD_LENGTH_BY_CHAR + 3;
    const START_TOP: usize = 2; // 2 lines down from the top of the terminal
    const WIDTH_OF_COLUMN: usize = 5;
    const WIDTH_OF_COLOUR_COLUMN: usize = 3;

    let number_of_players = players.len();

    // Top table header showing the columns of every player in the game
    print!("\x1B[{START_TOP};{START_LEFT}H");
    print!("{: <WIDTH_OF_COLOUR_COLUMN$}|", "");
    for player in players {
        print!(
            "{}{:^WIDTH_OF_COLUMN$}\x1b[0m|",
            player.colour, player.avatar
        );
    }

    // Border separating the top table header from the data columns
    print!(
        "\x1B[{};{START_LEFT}H{:=<width$}",
        START_TOP + 1,
        "=",
        width = WIDTH_OF_COLOUR_COLUMN + 1,
    );
    for _ in 0..number_of_players {
        print!("======");
    }

    // Show the amount of money each player has
    print!("\x1B[{};{START_LEFT}H$$$|", START_TOP + 2);
    for player in players {
        print!("{:>WIDTH_OF_COLUMN$}|", player.money);
    }

    // This is why we use a BTreeMap to store which properties each player owns
    // We do not want the order of sets to switch every inventory refresh, it would
    // be too confusing.
    for (line, (set, _)) in ownership_records[0].iter().enumerate() {
        print!("\x1B[{};{START_LEFT}H", line + START_TOP + 3);
        print!("{}   \x1b[0m|", colours::bg_set_colour(set.as_str()));
        for player in 0..number_of_players {
            let number_of_tiles_of_a_set_owned: usize = ownership_records[player][set].len();
            if number_of_tiles_of_a_set_owned == 0 {
                print!("{:>WIDTH_OF_COLUMN$}|", "");
            } else {
                print!("{:>WIDTH_OF_COLUMN$}|", number_of_tiles_of_a_set_owned);
            }
        }
    }
}

pub fn clear_inside_board() {
    const START_LEFT: usize = 26;
    for line in 7..33 {
        print!("\x1B[{line};{START_LEFT}H{: <38}", " ");
    }
}

pub fn display_board_tile(board_tile: &BoardTile) {
    const START_LEFT: usize = 26;

    clear_inside_board();
    match board_tile {
        BoardTile::Street(property) => print!("{}", property.display_card(START_LEFT, 11)),
        BoardTile::Railroad(property) => print!("{}", property.display_card(START_LEFT, 17)),
        BoardTile::Utility(property) => print!("{}", property.display_card(START_LEFT, 19)),
        BoardTile::Event(tile) => print!("{}", tile.display_card(START_LEFT, 25)),
        _ => todo!(),
    }
}

pub fn display_trading_desk(
    proposer_avatar: char,
    proposer_money: i64,
    proposer_tiles: &BTreeMap<usize, String>,
    receiver_avatar: char,
    receiver_money: i64,
    receiver_tiles: &BTreeMap<usize, String>,
) {
    const CARD_WIDTH: usize = 37;
    const WHITE_BACKGROUND: &str = "\x1b[47m";
    const BLACK_FOREGROUND: &str = "\x1b[30m";

    for row in 7..19 {
        print!(
            "\x1B[{row};27H{WHITE_BACKGROUND}{BLACK_FOREGROUND}{empty: ^CARD_WIDTH$}",
            empty = " "
        );
    }

    print!(
        "\x1B[7;27H{: ^CARD_WIDTH$}",
        format!("Player {} Offerings (ITEM_ID: ITEM)", proposer_avatar)
    );

    if proposer_money != 0 {
        print!("\x1B[8;27H  100: ${proposer_money}");
    }

    for (i, (tile_id, tile_name)) in proposer_tiles.iter().enumerate() {
        print!("\x1B[{row};27H  {tile_id}: {tile_name}", row = i + 9);
    }
    print!("\x1B[0m");

    for row in 20..32 {
        print!(
            "\x1B[{row};27H{WHITE_BACKGROUND}{BLACK_FOREGROUND}{empty: ^CARD_WIDTH$}",
            empty = " "
        );
    }

    print!(
        "\x1B[20;27H{: ^CARD_WIDTH$}",
        format!("Player {} Offerings (ITEM_ID: ITEM)", receiver_avatar)
    );

    if receiver_money != 0 {
        print!("\x1B[21;27H  100: ${receiver_money}");
    }

    for (i, (tile_id, tile_name)) in receiver_tiles.iter().enumerate() {
        print!("\x1B[{row};27H  {tile_id}: {tile_name}", row = i + 22);
    }
    print!("\x1B[0m");
}
