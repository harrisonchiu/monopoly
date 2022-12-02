// "Differences in property definitions: rent, rent_multiplier, house_cost, hotel_cost"
// "Same between property definitions: name, type, set, mortgage_value, property_cost"
// "Same between all tile definitions: name, type, set"
// Missing fields for ALL tiles
pub const JSON_MISSING_NAME: &str = "All JSON tile definitions must have a \
    <name> field with a string value";
pub const JSON_MISSING_TYPE: &str = "All JSON tile definitions must have a \
    <type> field with a string value";
pub const JSON_MISSING_SET: &str = "All JSON tile definitions must have a \
    <set> field with a string value";

// Missing fields for Property tiles
pub const JSON_MISSING_MORTGAGE_VALUE: &str = "All JSON property tile definitions must have a \
    <mortgage_value> field with an int value";
pub const JSON_MISSING_PROPERTY_COST: &str = "All JSON property tile definitions must have a \
    <property_cost> field with an int value";
pub const JSON_MISSING_RENT: &str = "All JSON property tile definitions must have a \
    <rent> field with an object of fields";
pub const JSON_MISSING_RENT_OBJECT_FIELDS: &str = "For a JSON property tile definition, \
    error in fetching some fields inside the <rent> object";
pub const JSON_MISSING_RENT_MULTIPLIER: &str = "All JSON tile definitions of <Utility> type \
    must have a <rent_multiplier> field with an object of fields";
pub const JSON_MISSING_RENT_MULTIPLIER_OBJECT_FIELDS: &str = "All JSON tile definitions of \
    <Utility> type must have a <rent_multiplier> field with an object of fields";
pub const JSON_MISSING_HOUSE_COST: &str = "All JSON tile definitions of <Street> type \
    must have a <house_cost> field with an int value";
pub const JSON_MISSING_HOTEL_COST: &str = "All JSON tile definitions of <Street> type \
    must have a <hotel_cost> field with an int value";

// Missing fields for Event tiles
pub const JSON_MISSING_MONEY_EVENT: &str = "All JSON tile definitions of <Event> type \
    must have a <money_event> field with an int value";

// Conversion error
pub const JSON_DESERIALIZE_TO_I64: &str = "Error in deserializing value into an i64. Likely \
    incorrect data type inputted for a field. Read JSON documentation for field and its data type";
pub const JSON_DESERIALIZE_TO_STR: &str = "Error in deserializing value into a str. Likely \
    incorrect data type inputted for a field. Read JSON documentation for field and its data type";
