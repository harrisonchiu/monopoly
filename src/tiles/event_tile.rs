pub struct EventTile {
    pub id: usize,
    pub name: String,
    pub set_name: String,
}

impl EventTile {
    pub fn new(id: usize, tile_data: &serde_json::Value) -> Self {
        Self {
            id: id,
            name: tile_data["name"].to_string(),
            set_name: tile_data["set"].to_string(),
        }
    }
}
