use board;
use board_tiles::tile::Event;
use colour::Colour;
use player::Avatar;

pub struct Go {
    id: usize,
    name: String,
    display_name: String,
    group: String,
    colour: String,
    landed_players: [bool; 4],
}

impl Event for Go {
    fn new(tile_id: usize, tile_data: &serde_json::Value) -> Self {
        Self {
            id: tile_id,
            name: tile_data["name"].as_str().unwrap().to_string(),
            display_name: tile_data["display_name"].as_str().unwrap().to_string(),
            group: tile_data["set"].as_str().unwrap().to_string(),
            colour: Colour::background(tile_data["set"].as_str().unwrap()).to_string(),
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

    fn get_details_row(&self) -> String {
        String::from("       \x1b[00000049m\x1b[0m")
    }
}
