pub fn colour(colour: &str) -> &str {
    match colour {
        "Red" => "\x1b[48;5;009m", // \x1b[41m
        "Orange" => "\x1b[48;5;202m",
        "Yellow" => "\x1b[48;5;011m",
        "Green" => "\x1b[48;5;010m",
        "Cyan" => "\x1b[48;5;014m",
        "Blue" => "\x1b[48;5;012m",
        "Magenta" => "\x1b[48;5;013m",           //\x1b[45m
        "Brown" => "\x1b[48;5;094m",             // \x1b[48;5;94m
        "Gray" | "Railroad" => "\x1b[48;5;008m", // Gray
        "White" | "Utility" => "\x1b[48;5;015m", // White
        _ => "\x1b[00000049m",                   // Default terminal colour
    }
}

pub struct Tile {
    pub display_name: String,
    pub colour: String,
}

impl Tile {
    pub fn new(tile_data: &serde_json::Value) -> Self {
        Self {
            display_name: tile_data["display_name"].as_str().unwrap().to_string(),
            colour: colour(tile_data["set"].as_str().unwrap()).to_string(),
        }
    }
}
