#include <nlohmann/json.hpp>

struct BoardConfig {
  using json = nlohmann::json;

private:
  static constexpr std::string_view classic_board{ R"""([
    {
        "name": "Go",
        "display_name": "GO",
        "type": "Corner",
        "group": "Black"
    },
    {
        "name": "Mediterranean Avenue",
        "display_name": "MEDIT AVE",
        "type": "Street",
        "group": "Brown",
        "cost": 60
    },
    {
        "name": "Community Chest 1",
        "display_name": "CHEST",
        "type": "Chest",
        "group": "Salmon"
    },
    {
        "name": "Baltic Avenue",
        "display_name": "BALTIC AVE",
        "type": "Street",
        "group": "Brown",
        "cost": 60
    },
    {
        "name": "Income Tax",
        "display_name": "INCOME TAX",
        "type": "Tax",
        "group": "Purple"
    },
    {
        "name": "Reading Railroad",
        "display_name": "READING RR",
        "type": "Railroad",
        "group": "Gray",
        "cost": 200
    },
    {
        "name": "Oriental Avenue",
        "display_name": "ORNTL AVE",
        "type": "Street",
        "group": "Cyan",
        "cost": 100
    },
    {
        "name": "Chance 1",
        "display_name": "CHANCE?",
        "type": "Chance",
        "group": "Spring"
    },
    {
        "name": "Vermont Avenue",
        "display_name": "VERMONT AVE",
        "type": "Street",
        "group": "Cyan",
        "cost": 100
    },
    {
        "name": "Connecticut Avenue",
        "display_name": "CONNECT AVE",
        "type": "Street",
        "group": "Cyan",
        "cost": 120
    },
    {
        "name": "Visiting Jail",
        "display_name": "VISIT JAIL",
        "type": "Corner",
        "group": "Black"
    },
    {
        "name": "St. Charles Place",
        "display_name": "CHARLES PLACE",
        "type": "Street",
        "group": "Magenta",
        "cost": 140
    },
    {
        "name": "Electric Company",
        "display_name": "ELEC COMPANY",
        "type": "Utility",
        "group": "White",
        "cost": 150
    },
    {
        "name": "States Avenue",
        "display_name": "STATES AVE",
        "type": "Street",
        "group": "Magenta",
        "cost": 140
    },
    {
        "name": "Virginia Avenue",
        "display_name": "VIRGNIA AVE",
        "type": "Street",
        "group": "Magenta",
        "cost": 160
    },
    {
        "name": "Pennsylvania Railroad",
        "display_name": "PENN RR",
        "type": "Railroad",
        "group": "Gray",
        "cost": 200
    },
    {
        "name": "St. James Place",
        "display_name": "JAMES PLACE",
        "type": "Street",
        "group": "Orange",
        "cost": 180
    },
    {
        "name": "Community Chest 2",
        "display_name": "CHEST",
        "type": "Chest",
        "group": "Salmon"
    },
    {
        "name": "Tennessee Avenue",
        "display_name": "TENN AVE",
        "type": "Street",
        "group": "Orange",
        "cost": 180
    },
    {
        "name": "New York Avenue",
        "display_name": "NEWYORK AVE",
        "type": "Street",
        "group": "Orange",
        "cost": 200
    },
    {
        "name": "Free Parking",
        "display_name": "FREE PARK",
        "type": "Corner",
        "group": "Black"
    },
    {
        "name": "Kentucky Avenue",
        "display_name": "KNTUCKY AVE",
        "type": "Street",
        "group": "Red",
        "cost": 220
    },
    {
        "name": "Chance 2",
        "display_name": "CHANCE?",
        "type": "Chance",
        "group": "Spring"
    },
    {
        "name": "Indiana Avenue",
        "display_name": "INDIANA AVE",
        "type": "Street",
        "group": "Red",
        "cost": 220
    },
    {
        "name": "Illinois Avenue",
        "display_name": "ILLNOIS AVE",
        "type": "Street",
        "group": "Red",
        "cost": 240
    },
    {
        "name": "B. & O. Railroad",
        "display_name": "B&O RR",
        "type": "Railroad",
        "group": "Gray",
        "cost": 200
    },
    {
        "name": "Atlantic Avenue",
        "display_name": "ATLANTC AVE",
        "type": "Street",
        "group": "Yellow",
        "cost": 260
    },
    {
        "name": "Ventnor Avenue",
        "display_name": "VENTNOR AVE",
        "type": "Street",
        "group": "Yellow",
        "cost": 260
    },
    {
        "name": "Water Works",
        "display_name": "WATER WORKS",
        "type": "Utility",
        "group": "White",
        "cost": 150
    },
    {
        "name": "Marvin Gardens",
        "display_name": "MARVIN GARDENS",
        "type": "Street",
        "group": "Yellow",
        "cost": 280
    },
    {
        "name": "Go To Jail",
        "display_name": "GOTO JAIL",
        "type": "Corner",
        "group": "Black"
    },
    {
        "name": "Pacific Avenue",
        "display_name": "PACIFIC AVE",
        "type": "Street",
        "group": "Green",
        "cost": 300
    },
    {
        "name": "North Carolina Avenue",
        "display_name": "CAROLNA AVE",
        "type": "Street",
        "group": "Green",
        "cost": 300
    },
    {
        "name": "Community Chest 3",
        "display_name": "CHEST",
        "type": "Chest",
        "group": "Salmon"
    },
    {
        "name": "Pennsylvania Avenue",
        "display_name": "PENN AVE",
        "type": "Street",
        "group": "Green",
        "cost": 320
    },
    {
        "name": "Short Railroad",
        "display_name": "SHORT RR",
        "type": "Railroad",
        "group": "Gray",
        "cost": 200
    },
    {
        "name": "Chance 3",
        "display_name": "CHANCE?",
        "type": "Chance",
        "group": "Spring"
    },
    {
        "name": "Park Place",
        "display_name": "PARK PLACE",
        "type": "Street",
        "group": "Blue",
        "cost": 350
    },
    {
        "name": "Luxury Tax",
        "display_name": "LUXURY TAX",
        "type": "Tax",
        "group": "Purple"
    },
    {
        "name": "Boardwalk",
        "display_name": "BRDWALK",
        "type": "Street",
        "group": "Blue",
        "cost": 400
    }
])""" };

  json board;

public:
  BoardConfig()
      : board{ classic_board } {};
};