use event_tile;
use railroad_tile;
use street_tile;
use utility_tile;

pub enum BoardTile {
    StreetTile(street_tile::StreetTile),
    RailroadTile(railroad_tile::RailroadTile),
    UtilityTile(utility_tile::UtilityTile),
    EventTile(event_tile::EventTile),
}

pub enum PropertyStatus {
    // Keeps track of: whether the tile is owned, the buildings, and mortgage
    // Implement rent and building rules within each tile struct
    // Ex: Utilities should not have Tier3 and above
    // Ex: If property is Tier1 and below, the property is tradable
    Mortgage = -1,
    Unowned = 0,
    Tier1 = 1, // Basic
    Tier2 = 2, // 1 house | 2 of the set owned
    Tier3 = 3, // 2 houses | 3 of the set owned
    Tier4 = 4, // 3 houses | 4 of the set owned
    Tier5 = 5, // 4 houses | 5 of the set owned
    Tier6 = 6, // Hotel | 6 of the set owned
}

impl PropertyStatus {
    fn is_status_x_less_than_status_y(status_x: PropertyStatus, status_y: PropertyStatus) -> bool {
        if (status_x as i8) < (status_y as i8) {
            true
        } else {
            false
        }
    }
}
