#include "src/model/tiles/tile.hpp"

// #include "src/model/player.hpp"
// #include "src/model/tiles/attributes.hpp"

Property::Property(const json &tile_data, const int id) : Tile(tile_data, id) {}

// Interacting with a @Property tile is attempting to buy that tile
