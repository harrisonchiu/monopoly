use constants;
use error;

pub struct EventTile {
    info: serde_json::Value,
}

impl EventTile {
    pub fn new(tile_data: serde_json::Value) -> Self {
        Self { info: tile_data }
    }

    pub fn get_tile_name(&self) -> String {
        self.info
            .get("name")
            .expect(error::JSON_MISSING_NAME)
            .to_string()
    }

    fn get_set_name(&self) -> &str {
        // Must return &str to easily fetch from Map<&str, &str>. Conversion seems to
        // keep quotes in the str? which the keys obviously do not have so it fails to fetch.
        // All JSON definitions must have a set field, so this should return str without fail
        self.info
            .get("set")
            .expect(error::JSON_MISSING_SET)
            .as_str()
            .expect(error::JSON_DESERIALIZE_TO_STR)
    }

    pub fn get_set_colour_string(&self) -> &str {
        // The top row (same row as ▔ top border) with background colour of the tile's set
        // or no background colour. It does not affect foreground colour of ▔
        constants::SET_NAME_TO_COLOUR_STRING
            .get(self.get_set_name())
            .unwrap_or(&constants::DEFAULT_COLOUR_STRING)
    }
}
