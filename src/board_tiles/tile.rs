use player::Avatar;

pub enum Tile {
    Property(Box<dyn Property>),
    Event(Box<dyn Event>),
}

impl Tile {
    pub fn get_display_name(&self) -> &String {
        match self {
            Self::Property(t) => t.get_display_name(),
            Self::Event(t) => t.get_display_name(),
        }
    }

    pub fn get_colour(&self) -> &String {
        match self {
            Self::Property(t) => t.get_colour(),
            Self::Event(t) => t.get_colour(),
        }
    }

    pub fn get_details_row(&self) -> String {
        match self {
            Self::Property(t) => t.get_details_row(),
            Self::Event(t) => t.get_details_row(),
        }
    }
}

pub trait Event {
    fn new(tile_id: usize, tile_data: &serde_json::Value) -> Self
    where
        Self: Sized;

    fn get_name(&self) -> &String;
    fn get_display_name(&self) -> &String;
    fn get_colour(&self) -> &String;
    fn get_details_row(&self) -> String;
}

pub trait Property {
    fn new(tile_id: usize, tile_data: &serde_json::Value) -> Self
    where
        Self: Sized;

    fn get_name(&self) -> &String;
    fn get_display_name(&self) -> &String;
    fn get_colour(&self) -> &String;
    fn get_owner(&self) -> &Option<Avatar>;
    fn get_details_row(&self) -> String;
}
