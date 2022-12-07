use board;
use error;
use player;

use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;

use tiles::{
    board_tile::BoardTile, event_tile::EventTile, railroad_tile::RailroadTile,
    street_tile::StreetTile, utility_tile::UtilityTile,
};

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

pub fn roll_dice(die_range: &Uniform<u8>, die_1: &mut StdRng, die_2: &mut StdRng) -> [u8; 2] {
    // We split it up to make it more extendible in the future
    // That is we could have an option to use a singular die [0, 12] for an equal
    // distribution (which would change the game a lot) and make it easier to
    // display the value of each die (draw each die value instead of just printing the sum)
    [die_range.sample(die_1), die_range.sample(die_2)]
}

pub fn is_doubles(dice: &[u8; 2]) -> bool {
    dice[0] == dice[1]
}

pub fn purchase_tile(mut player: player::Player, tile: &mut BoardTile) -> bool {
    match tile {
        BoardTile::Street(property) => {
            player.pay(500);
            property.acquired_by(player);
        }
        BoardTile::Railroad(property) => {
            player.pay(property.property_cost);
            // property.acqui
        }
        _ => todo!(),
    }
    false
}
