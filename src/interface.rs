use board;
use colours;
use game::PropertySet;
use player::Player;
use tiles::{
    board_tile::BoardTile, event_tile::EventTile, railroad_tile::RailroadTile,
    street_tile::StreetTile, utility_tile::UtilityTile,
};

pub const INSTRUCTIONS_MOVE: &str =
    "    | [1] Roll/Move | [2] Buy Property | [3] View a Property | [4] Trade | [5] Quit |";
pub const INSTRUCTIONS_END_TURN: &str =
    "    | [1] End Turn | [2] Buy Property | [3] View a Property | [4] Trade | [5] Quit |";

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

pub fn display_board_tile(board_tile: &BoardTile) {
    const START_LEFT: usize = 26;

    fn clear_inside_board() {
        for line in 7..33 {
            print!("\x1B[{line};{START_LEFT}H{: <38}", " ");
        }
    }

    clear_inside_board();
    match board_tile {
        BoardTile::Street(property) => print!("{}", property.display_card(START_LEFT, 11)),
        BoardTile::Railroad(property) => print!("{}", property.display_card(START_LEFT, 17)),
        BoardTile::Utility(property) => print!("{}", property.display_card(START_LEFT, 19)),
        BoardTile::Event(tile) => print!("{}", tile.display_card(START_LEFT, 25)),
        _ => todo!(),
    }
}
