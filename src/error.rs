/*!
 * The following are a cumulation of error messages that may occur throughout the
 * lifetime of the program. They are all here to easily see all possible errors and
 * to easily write consistent error messages. The majority of these are JSON attribute errors.
 * The errors are usually used inside `.expect()` as this program does not usually handle
 * errors by defaulting to a value, but rather panicking and terminating the program with
 * an error message. This is because the majority of errors will occur from reading JSON files
 * containing data for tiles, cards, and settings. Implicitly, this means the program NEEDS
 * to have those JSON files written correctly as per documentation and assumes it to be so.
 * These are only here to help the user fix the errors.
 */
use serde_json;

use board;

// JSON structure errors
pub const JSON_DESERIALIZE: &str = "JSON could not be deserialized because of an error. \
    It likely has a bad format or structure. Also ensure JSON is structured as an array of objects";
const JSON_NON_ARRAY_TYPE: &str = "JSON must be structured as an array of objects \
    with the objects placed in the order of the start tile to the last tile before start tile";
const JSON_EXISTENCE_OF_DATA: &str = "Error in deserializing JSON. Could not even find \
    first item. Likely bad structure or format.";
const JSON_METADATA_OBJECT_TYPE: &str = "First item in JSON array must be an object \
    of metadata with a <__documentation> field. This is solely for needed documentation";
const JSON_INCORRECT_NUMBER_OF_DATA: &str = "JSON array of objects must have \
    41 objects (40 tiles + 1 metadata) inside the array with the first object being the metadata";

// Missing attributes for all tiles
const JSON_MISSING_NAME: &str = "All JSON tile definitions must have a \
    <name> field with a string value";
const JSON_MISSING_TYPE: &str = "All JSON tile definitions must have a \
    <type> field with a string value";
const JSON_MISSING_SET: &str = "All JSON tile definitions must have a \
    <set> field with a string value";

// Missing attribute for all property tiles
const JSON_MISSING_PROPERTY_COST: &str = "All JSON property tile definitions must have a \
    <property_cost> field with an int value";
const JSON_MISSING_MORTGAGE_VALUE: &str = "All JSON property tile definitions must have a \
    <mortgage_value> field with an int value";

// Missing attirbutes for street tiles
pub const JSON_STREET_MISSING_RENT: &str = "All JSON Street tile definitions must have a \
    <rent> field with an object of 6 str keys and 6 int values";
const JSON_MISSING_HOUSE_COST: &str = "All JSON tile definitions of <Street> type \
    must have a <house_cost> field with an int value";
const JSON_MISSING_HOTEL_COST: &str = "All JSON tile definitions of <Street> type \
    must have a <hotel_cost> field with an int value";

// Missing attributes for railroad tiles
pub const JSON_RAILROAD_MISSING_RENT: &str = "All JSON Railroad tile definitions must have a \
    <rent> field with an object of 4 str keys and 4 int values";

// Missing attributes for utility tiles
pub const JSON_UTILITY_MISSING_RENT: &str = "All JSON Utility tile definitions must have a \
    <rent_multiplier> field with an object of 2 str keys and 2 int values";

// Missing attributes for event tiles
const JSON_MISSING_MONEY_EVENT: &str = "All JSON tile definitions of <Event> type \
    must have a <money_event> field with an int value";

// Conversion error
pub const JSON_DESERIALIZE_TO_I64: &str = "Error in deserializing value into an i64. Likely \
    incorrect data type inputted for a field. Read JSON documentation for field and its data type";
pub const JSON_DESERIALIZE_TO_STR: &str = "Error in deserializing value into a str. Likely \
    incorrect data type inputted for a field. Read JSON documentation for field and its data type";

pub fn validate_tile_data_json(json: &serde_json::Value, verbose: bool) -> bool {
    //! NOTE: this function does not check if rent objects are in non-decreasing order
    //! It is too difficult to validate for it right now given the JSON structure.
    //! The game assumes that as rent level increases so does the rent cost
    //! Ex: rent_tier1 <= rent_tier2 <= rent_tier3, where rent_tier3 is the highest level

    let mut tile_type: &str; // Different tile types need various fields and thus various checks
    let mut tile_name: &str; // Tile name is used to more precisely alert where the error occured

    // JSON must have 40 objects for the tile
    let all_tiles: &Vec<serde_json::Value> = json.as_array().expect(JSON_NON_ARRAY_TYPE);
    if all_tiles.len() != board::BOARD_TOTAL_NUMBER_OF_TILES + 1 {
        panic!("{}", JSON_INCORRECT_NUMBER_OF_DATA);
    }

    // First item must be metadata, containing the field <__documentation> (information) for the user
    all_tiles
        .first()
        .expect(JSON_EXISTENCE_OF_DATA)
        .as_object()
        .expect(JSON_METADATA_OBJECT_TYPE)
        .get("__documentation")
        .expect(JSON_METADATA_OBJECT_TYPE);

    // Check all the tiles data after the documentation
    for tile in all_tiles.iter().skip(1) {
        // Every tile must have: name, type, set
        tile_name = tile
            .get("name")
            .expect(JSON_MISSING_NAME)
            .as_str()
            .expect(&format!("{tile:?} name - {}", JSON_DESERIALIZE_TO_STR));

        if verbose {
            println!("Checking object with name: {}", tile_name);
        }

        tile_type = tile
            .get("type")
            .expect(&format!("{tile_name} type - {}", JSON_MISSING_TYPE))
            .as_str()
            .expect(&format!("{tile_name} type - {}", JSON_DESERIALIZE_TO_STR));
        tile.get("set")
            .expect(&format!("{tile_name} set - {}", JSON_MISSING_SET))
            .as_str()
            .expect(&format!("{tile_name} set - {}", JSON_DESERIALIZE_TO_STR));
        match tile_type {
            "Street" => {
                tile.get("mortgage_value")
                    .expect(&format!(
                        "{tile_name} mortgage_value - {}",
                        JSON_MISSING_MORTGAGE_VALUE
                    ))
                    .as_i64()
                    .expect(&format!(
                        "{tile_name} mortgage_value - {}",
                        JSON_DESERIALIZE_TO_I64
                    ));
                let rent = tile
                    .get("rent")
                    .expect(&format!("{tile_name} rent - {}", JSON_STREET_MISSING_RENT))
                    .as_object()
                    .expect(&format!("{tile_name} rent - {}", JSON_STREET_MISSING_RENT));
                if rent.len() != 6 {
                    panic!("{tile_name} {}", JSON_STREET_MISSING_RENT)
                }
                rent.iter().for_each(|(_, amount)| {
                    amount
                        .as_i64()
                        .expect(&format!("{tile_name} rent - {}", JSON_STREET_MISSING_RENT));
                });
                tile.get("property_cost")
                    .expect(&format!(
                        "{tile_name} property_cost - {}",
                        JSON_MISSING_PROPERTY_COST
                    ))
                    .as_i64()
                    .expect(&format!(
                        "{tile_name} property_cost - {}",
                        JSON_DESERIALIZE_TO_I64
                    ));
                tile.get("house_cost")
                    .expect(&format!(
                        "{tile_name} house_cost - {}",
                        JSON_MISSING_HOUSE_COST
                    ))
                    .as_i64()
                    .expect(&format!(
                        "{tile_name} house_cost - {}",
                        JSON_DESERIALIZE_TO_I64
                    ));
                tile.get("hotel_cost")
                    .expect(&format!(
                        "{tile_name} hotel_cost - {}",
                        JSON_MISSING_HOTEL_COST
                    ))
                    .as_i64()
                    .expect(&format!(
                        "{tile_name} hotel_cost - {}",
                        JSON_DESERIALIZE_TO_I64
                    ));
            }
            "Railroad" => {
                tile.get("mortgage_value")
                    .expect(&format!(
                        "{tile_name} mortgage_value - {}",
                        JSON_MISSING_MORTGAGE_VALUE
                    ))
                    .as_i64()
                    .expect(&format!(
                        "{tile_name} mortgage_value - {}",
                        JSON_DESERIALIZE_TO_I64
                    ));
                let rent = tile
                    .get("rent")
                    .expect(&format!(
                        "{tile_name} rent - {}",
                        JSON_RAILROAD_MISSING_RENT
                    ))
                    .as_object()
                    .expect(&format!(
                        "{tile_name} rent - {}",
                        JSON_RAILROAD_MISSING_RENT
                    ));
                if rent.len() != 4 {
                    panic!("{tile_name} {}", JSON_RAILROAD_MISSING_RENT)
                }
                rent.iter().for_each(|(_, amount)| {
                    amount.as_i64().expect(&format!(
                        "{tile_name} rent - {}",
                        JSON_RAILROAD_MISSING_RENT
                    ));
                });
                tile.get("property_cost")
                    .expect(&format!(
                        "{tile_name} property_cost - {}",
                        JSON_MISSING_PROPERTY_COST
                    ))
                    .as_i64()
                    .expect(&format!(
                        "{tile_name} property_cost - {}",
                        JSON_DESERIALIZE_TO_I64
                    ));
            }
            "Utility" => {
                tile.get("mortgage_value")
                    .expect(&format!(
                        "{tile_name} mortgage_value - {}",
                        JSON_MISSING_MORTGAGE_VALUE
                    ))
                    .as_i64()
                    .expect(&format!(
                        "{tile_name} mortgage_value - {}",
                        JSON_DESERIALIZE_TO_I64
                    ));
                let rent = tile
                    .get("rent_multiplier")
                    .expect(&format!(
                        "{tile_name} rent_multiplier - {}",
                        JSON_UTILITY_MISSING_RENT
                    ))
                    .as_object()
                    .expect(&format!(
                        "{tile_name} rent_multiplier - {}",
                        JSON_UTILITY_MISSING_RENT
                    ));
                if rent.len() != 2 {
                    panic!("{tile_name} {}", JSON_UTILITY_MISSING_RENT)
                }
                rent.iter().for_each(|(_, amount)| {
                    amount.as_i64().expect(&format!(
                        "{tile_name} rent_multiplier - {}",
                        JSON_UTILITY_MISSING_RENT
                    ));
                });
                tile.get("property_cost")
                    .expect(&format!(
                        "{tile_name} property_cost - {}",
                        JSON_MISSING_PROPERTY_COST
                    ))
                    .as_i64()
                    .expect(&format!(
                        "{tile_name} property_cost - {}",
                        JSON_DESERIALIZE_TO_I64
                    ));
            }
            "Event" => {
                tile.get("money_event")
                    .expect(&format!(
                        "{tile_name} money_event - {}",
                        JSON_MISSING_MONEY_EVENT
                    ))
                    .as_i64()
                    .expect(&format!(
                        "{tile_name} money_event - {}",
                        JSON_DESERIALIZE_TO_I64
                    ));
            }
            _ => panic!(
                "Unknown type! Only 4 supported tile types: Street, Railroad, Utility, Event"
            ),
        }
    }
    true
}
