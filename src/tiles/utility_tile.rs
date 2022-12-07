use error;
use tiles::board_tile::PropertyStatus;

pub struct UtilityTile {
    pub info: serde_json::Value,
    property_status: PropertyStatus,
    property_cost: i64,
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
            property_cost: tile_data
                .get("property_cost")
                .expect(error::JSON_MISSING_PROPERTY_COST)
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

    fn get_rent(&self, level: &str) -> i64 {
        self.info
            .get("rent_multiplier")
            .expect(error::JSON_MISSING_RENT_MULTIPLIER)
            .get(level)
            .expect(error::JSON_MISSING_RENT_MULTIPLIER_OBJECT_FIELDS)
            .as_i64()
            .expect(error::JSON_DESERIALIZE_TO_I64)
    }

    fn update_current_rent(&mut self) {
        // Mortgaging the property or buying and selling buildings affect the rent.
        // Info for these actions are based on the <property_status> field
        // We could implement this with fields for number of houses and hotels, but
        // there would be more edge cases on which combinations would be rule breaking.
        // It is easier to work with buildings as a tier: see board_tile::PropertyStatus

        match self.property_status {
            PropertyStatus::Mortgaged => self.rent_multiplier = 0,
            PropertyStatus::Unowned => self.rent_multiplier = 0,
            PropertyStatus::Owned => self.rent_multiplier = self.get_rent("basic"),
            PropertyStatus::Tier1 => self.rent_multiplier = self.get_rent("2_utility"),
            _ => println!("Breaking the rules likely due to error in game logic"),
        }
    }

    pub fn get_property_information_string(&self) -> String {
        match self.property_status {
            PropertyStatus::Mortgaged => format!("|MRTGAGE|"),
            PropertyStatus::Unowned => format!("${}", self.property_cost.to_string()),
            PropertyStatus::Owned => format!("x{} X", self.rent_multiplier),
            PropertyStatus::Tier1 => format!("x{} X", self.rent_multiplier),
            _ => format!("ERROR"),
        }
    }
}
