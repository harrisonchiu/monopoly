use board_tile::PropertyStatus;
use constants;
use error;

pub struct UtilityTile {
    info: serde_json::Value,
    property_status: PropertyStatus,
    rent_multiplier: i64,
}

impl UtilityTile {
    pub fn new(tile_data: serde_json::Value) -> Self {
        Self {
            property_status: PropertyStatus::Unowned,
            rent_multiplier: tile_data
                .get("rent_multiplier")
                .expect(error::JSON_MISSING_RENT_MULTIPLIER)
                .get("basic")
                .expect(error::JSON_MISSING_RENT_MULTIPLIER_OBJECT_FIELDS)
                .as_i64()
                .expect(error::JSON_DESERIALIZE_TO_I64),
            info: tile_data,
        }
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

    fn update_current_rent(&mut self) {
        // Mortgaging the property or buying and selling buildings affect the rent.
        // Info for these actions are based on the <property_status> field
        fn get_rent_level(info: &serde_json::Value, level: &str) -> i64 {
            info.get("rent_multiplier")
                .expect(error::JSON_MISSING_RENT_MULTIPLIER)
                .get(level)
                .expect(error::JSON_MISSING_RENT_MULTIPLIER_OBJECT_FIELDS)
                .as_i64()
                .expect(error::JSON_DESERIALIZE_TO_I64)
        }

        match self.property_status {
            PropertyStatus::Mortgage => self.rent_multiplier = 0,
            PropertyStatus::Unowned => self.rent_multiplier = 0,
            PropertyStatus::Tier1 => self.rent_multiplier = get_rent_level(&self.info, "basic"),
            PropertyStatus::Tier2 => self.rent_multiplier = get_rent_level(&self.info, "2_utility"),
            _ => println!("Breaking the rules likely due to error in game logic"),
        }
    }

    pub fn get_property_information_string(&self) -> String {
        match self.property_status {
            PropertyStatus::Mortgage => format!("|MRTGAGE|"),
            PropertyStatus::Unowned => format!(
                "${}",
                self.info
                    .get("property_cost")
                    .expect(error::JSON_MISSING_PROPERTY_COST)
                    .to_string(),
            ),
            PropertyStatus::Tier1 => format!("x{} X", self.rent_multiplier),
            PropertyStatus::Tier2 => format!("x{} X", self.rent_multiplier),
            _ => format!("ERROR"),
        }
    }
}
