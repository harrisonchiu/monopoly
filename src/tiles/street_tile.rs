use error;
use tiles::board_tile::PropertyStatus;

pub struct StreetTile {
    pub owner: Option<usize>,
    property_status: PropertyStatus,
    pub rent: i64,
    pub property_cost: i64,
    pub info: serde_json::Value,
}

impl StreetTile {
    pub fn new(tile_data: serde_json::Value) -> Self {
        Self {
            owner: None,
            property_status: PropertyStatus::Unowned,
            rent: tile_data
                .get("rent")
                .expect(error::JSON_MISSING_RENT)
                .get("basic")
                .expect(error::JSON_MISSING_RENT_OBJECT_FIELDS)
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

    pub fn acquired_by(&mut self, owner_id: usize) {
        self.owner = Some(owner_id);
        self.property_status = PropertyStatus::Owned;
        self.update_current_rent();
    }

    fn get_rent_from_level(&self, level: &str) -> i64 {
        self.info
            .get("rent")
            .expect(error::JSON_MISSING_RENT)
            .get(level)
            .expect(error::JSON_MISSING_RENT_OBJECT_FIELDS)
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
            PropertyStatus::Mortgaged => self.rent = 0,
            PropertyStatus::Unowned => self.rent = 0,
            PropertyStatus::Owned => self.rent = self.get_rent_from_level("basic"),
            PropertyStatus::Tier1 => self.rent = self.get_rent_from_level("1_house"),
            PropertyStatus::Tier2 => self.rent = self.get_rent_from_level("2_house"),
            PropertyStatus::Tier3 => self.rent = self.get_rent_from_level("3_house"),
            PropertyStatus::Tier4 => self.rent = self.get_rent_from_level("4_house"),
            PropertyStatus::Tier5 => self.rent = self.get_rent_from_level("hotel"),
        }
    }

    pub fn get_property_information_string(&self) -> String {
        match self.property_status {
            PropertyStatus::Mortgaged => format!("|MRTGAGE|"),
            PropertyStatus::Unowned => format!(" ${}", self.property_cost.to_string()),
            PropertyStatus::Owned => format!("${} X", self.rent),
            PropertyStatus::Tier1 => format!("${} 1", self.rent),
            PropertyStatus::Tier2 => format!("${} 2", self.rent),
            PropertyStatus::Tier3 => format!("${} 3", self.rent),
            PropertyStatus::Tier4 => format!("${} 4", self.rent),
            PropertyStatus::Tier5 => format!("${} H", self.rent),
        }
    }
}
