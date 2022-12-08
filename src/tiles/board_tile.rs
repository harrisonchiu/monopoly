use board;
use error;
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
    pub fn get_tile_name(&self) -> String {
        match self {
            BoardTile::Street(tile) => tile
                .info
                .get("name")
                .expect(error::JSON_MISSING_NAME)
                .to_string(),
            BoardTile::Railroad(tile) => tile
                .info
                .get("name")
                .expect(error::JSON_MISSING_NAME)
                .to_string(),
            BoardTile::Utility(tile) => tile
                .info
                .get("name")
                .expect(error::JSON_MISSING_NAME)
                .to_string(),
            BoardTile::Event(tile) => tile
                .info
                .get("name")
                .expect(error::JSON_MISSING_NAME)
                .to_string(),
        }
    }

    pub fn get_set_name(&self) -> &str {
        // Must return &str to easily fetch from Map<&str, &str>. Conversion seems to
        // keep quotes in the str which the keys obviously do not have so it fails to fetch.
        // All JSON definitions must have a set field, so this should return str without fail
        match self {
            BoardTile::Street(tile) => tile
                .info
                .get("set")
                .expect(error::JSON_MISSING_NAME)
                .as_str()
                .expect(error::JSON_DESERIALIZE_TO_STR),
            BoardTile::Railroad(tile) => tile
                .info
                .get("set")
                .expect(error::JSON_MISSING_NAME)
                .as_str()
                .expect(error::JSON_DESERIALIZE_TO_STR),
            BoardTile::Utility(tile) => tile
                .info
                .get("set")
                .expect(error::JSON_MISSING_NAME)
                .as_str()
                .expect(error::JSON_DESERIALIZE_TO_STR),
            BoardTile::Event(tile) => tile
                .info
                .get("set")
                .expect(error::JSON_MISSING_NAME)
                .as_str()
                .expect(error::JSON_DESERIALIZE_TO_STR),
        }
    }

    pub fn get_set_colour_string(&self) -> &str {
        // The top row (same row as ▔ top border) with background colour of the tile's set
        // or no background colour. It does not affect foreground colour of ▔
        board::COLOURED_REGION_OF_EACH_SET
            .get(self.get_set_name())
            .unwrap_or(&board::UNCOLOURED_REGION)
    }
}

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
