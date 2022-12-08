///! The difference between methods defined here and those methods in `game.rs` or `main.rs`
///! is that these methods are a wrapper for data and values inherent to the tiles.
///! `game.rs` define wrapper functions for all BoardTiles that are more action based
///! done by players.
///! `main.rs` runs the main game loop that uses the functions defined here and `game.rs`
///! It involves its own code but it is mostly for runnings these functions based on
///! the game's overarching rules or to display logs for the player.
use board;
use error;
use player;

use const_format;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;

use tiles::{
    board_tile::BoardTile, event_tile::EventTile, railroad_tile::RailroadTile,
    street_tile::StreetTile, utility_tile::UtilityTile,
};

// Maximmum of 4 players. Format of "FOREGROUND_COLOR AVATAR END_COLOUR"
// Each player avatar is differently coloured (foreground) with a different symbol
// ID MUST be in range [0, 3] because player drawing is based on id incrementing by 1 from 0
// Leave ID as index of this array in the main game loop where the players are created
pub const PLAYER_PIECES: [&'static str; 4] = [
    const_format::concatcp!("\x1b[96m", "A", "\x1b[0m"), // Light blue
    const_format::concatcp!("\x1b[92m", "B", "\x1b[0m"), // Light green
    const_format::concatcp!("\x1b[91m", "C", "\x1b[0m"), // Light red
    const_format::concatcp!("\x1b[95m", "D", "\x1b[0m"), // Light magenta
];

pub const INSTRUCTIONS_MAIN_MENU_ROLL: &'static str =
    "    | [1] Roll/Move | [2] Buy Property | [3] View a Property | [4] Trade | [5] Quit |";
pub const INSTRUCTIONS_MAIN_MENU_END_TURN: &'static str =
    "    | [1] End Turn | [2] Buy Property | [3] View a Property | [4] Trade | [5] Quit |";

pub fn create_board() -> board::Board {
    let tile_data_json: &str = include_str!("./tiles/board_tile_data.json");
    let json: serde_json::Value = serde_json::from_str(&tile_data_json)
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

pub fn roll_dice(die_range: &Uniform<i8>, die_1: &mut StdRng, die_2: &mut StdRng) -> [i8; 2] {
    //! We split it up to make it more extendible in the future
    //! That is we could have an option to use a singular die [0, 12] for an equal
    //! distribution (which would change the game a lot) and make it easier to
    //! display the value of each die (draw each die value instead of just printing the sum)
    [die_range.sample(die_1), die_range.sample(die_2)]
}

pub fn is_doubles(dice: &[i8; 2]) -> bool {
    dice[0] == dice[1]
}

pub fn try_to_buy_tile(buyer: &mut player::Player, tile: &mut BoardTile) -> Option<i64> {
    //! Makes the player spends it money to take ownership of a property tile
    //! Returns the price the player paid if allowed (not owned and is a property tile)
    //! Otherwise if unsuccessful, return None
    match tile {
        BoardTile::Street(property) if property.owner.is_none() => {
            buyer.pay(property.property_cost);
            property.acquired_by(buyer.id);
            Some(property.property_cost)
        }
        BoardTile::Railroad(property) if property.owner.is_none() => {
            buyer.pay(property.property_cost);
            property.acquired_by(buyer.id);
            Some(property.property_cost)
        }
        BoardTile::Utility(property) if property.owner.is_none() => {
            buyer.pay(property.property_cost);
            property.acquired_by(buyer.id);
            Some(property.property_cost)
        }
        _ => None,
    }
}
