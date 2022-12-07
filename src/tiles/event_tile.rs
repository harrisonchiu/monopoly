use error;

pub struct EventTile {
    pub info: serde_json::Value,
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
}
