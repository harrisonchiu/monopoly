pub struct EventTile {
    pub info: serde_json::Value,
}

impl EventTile {
    pub fn new(tile_data: serde_json::Value) -> Self {
        Self { info: tile_data }
    }
}
