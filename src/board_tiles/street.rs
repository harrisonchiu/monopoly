use board_tiles::tile::Property;
use colour::Colour;
use player::Avatar;

pub struct Street {
    id: usize,
    name: String,
    display_name: String,
    group: String,
    colour: String,
    owner: Option<Avatar>,
    landed_players: [bool; 4],
}

impl Property for Street {
    fn new(tile_id: usize, tile_data: &serde_json::Value) -> Self {
        Self {
            id: tile_id,
            name: tile_data["name"].as_str().unwrap().to_string(),
            display_name: tile_data["display_name"].as_str().unwrap().to_string(),
            group: tile_data["set"].as_str().unwrap().to_string(),
            colour: Colour::background(tile_data["set"].as_str().unwrap()).to_string(),
            owner: None,
            landed_players: [false; 4],
        }
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_display_name(&self) -> &String {
        &self.display_name
    }

    fn get_colour(&self) -> &String {
        &self.colour
    }

    fn get_owner(&self) -> &Option<Avatar> {
        &self.owner
    }

    fn get_details_row(&self) -> String {
        format!("${:<4} {}{}\x1b[0m", self.id, self.colour, "X")
    }
}
