#![allow(dead_code)]
use tiles::{event_tile, railroad_tile, street_tile, utility_tile};

pub enum BoardTile {
    Street(street_tile::StreetTile),
    Railroad(railroad_tile::RailroadTile),
    Utility(utility_tile::UtilityTile),
    Event(event_tile::EventTile),
}

impl BoardTile {
    //! This is like a parent class, apply methods to all child classes
    //! All tile structs that is grouped in the `enum BoardTile` should
    //! be able to run and return the code within the closures of each `match`
    //! i.e. the structs should run the equivalent of the inherited methods
    //!
    //! The difference between methods defined here and those methods in `game.rs` or `main.rs`
    //! is that these methods are a wrapper for data and values inherent to the tiles.
    //! `game.rs` define wrapper functions for all BoardTiles that are more action based
    //! done by players.
    //! `main.rs` runs the main game loop that uses the functions defined here and `game.rs`
    //! It involves its own code but it is mostly for runnings these functions based on
    //! the game's overarching rules or to display logs for the player.
}

#[derive(PartialEq)]
pub enum PropertyStatus {
    Mortgaged = -2,
    Unowned = -1,
    Owned = 0, // Basic rent | 1 owned of the set
    Tier1 = 1, // 1 house | 2 owned of the set
    Tier2 = 2, // 2 house | 3 owned of the set
    Tier3 = 3, // 3 house | 4 owned of the set
    Tier4 = 4, // 4 house | 5 owned of the set
    Tier5 = 5, // 5 house | 6 owned of the set
}

// #![allow(dead_code)]
pub fn bg_set_colour(colour: &str) -> String {
    //! Use
    match colour {
        "Red" => String::from("\x1b[41m"),
        "Orange" => String::from("\x1b[48;5;166m"),
        "Yellow" => String::from("\x1b[43m"),
        "Green" => String::from("\x1b[42m"),
        "Cyan" => String::from("\x1b[46m"),
        "Blue" => String::from("\x1b[44m"),
        "Magenta" => String::from("\x1b[45m"),
        "Brown" => String::from("\x1b[48;5;94m"),
        "Railroad" => String::from("\x1b[100m"), // Gray
        "Utility" => String::from("\x1b[47m"),   // White
        _ => panic!(
            "Unknown set name! We assume uses of this function \
            and its parameter are hardcoded and NOT dynamically inputted."
        ),
    }
}
