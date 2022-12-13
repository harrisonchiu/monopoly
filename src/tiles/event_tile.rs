pub struct EventTile {
    pub id: usize,
    pub name: String,
    pub set_name: String,
    pub colour: String,
}

impl EventTile {
    pub fn new(id: usize, tile_data: &serde_json::Value) -> Self {
        Self {
            id: id,
            name: tile_data["name"].as_str().unwrap().to_string(),
            set_name: tile_data["set"].as_str().unwrap().to_string(),
            colour: "".to_string(),
        }
    }
}
